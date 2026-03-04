# Android 打包签名配置指南

## 快速开始

### 1. 生成签名密钥

运行签名密钥生成脚本:

```bash
./scripts/setup_android_signing.sh
```

根据提示选择:
- **选项 1**: 只生成 Debug 密钥 (用于开发测试)
- **选项 2**: 只生成 Release 密钥 (用于生产发布,会自动配置密码)
- **选项 3**: 两者都生成 (推荐)

**重要**: 选择选项 2 或 3 时,脚本会:
1. 自动从 `keystore.env` 读取密码和密钥信息(如果已配置)
2. 如果配置文件不存在或密码无效,则提示输入
3. 完全自动化生成,无需手动交互

**推荐的完全自动化流程**:
1. 复制并编辑配置文件:
   ```bash
   cp build/android/keystore/keystore.env.template build/android/keystore/keystore.env
   # 编辑 keystore.env,填入密码和密钥信息
   ```
2. 运行脚本:
   ```bash
   ./scripts/setup_android_signing.sh  # 选择 2 或 3
   # 完全自动化,无需任何输入!
   ```

### 2. 构建 APK

```bash
# Debug 构建 (开发测试用)
./build/package/package.sh -p android -r debug

# Release 构建 (生产发布用)
./build/package/package.sh -p android -r release
```

---

## 详细说明

### 密钥类型区别

| 特性 | Debug 密钥 | Release 密钥 |
|------|-----------|-------------|
| **用途** | 开发测试 | 生产发布 |
| **密码** | 固定 (`android`) | 自定义 (需保密) |
| **位置** | `build/android/keystore/debug.keystore` | `build/android/keystore/release.keystore` |
| **Git 管理** | 可以提交 (可选) | **不要提交** |
| **别名** | `androiddebugkey` | `torn_trade` |
| **有效期** | 10000 天 (~27年) | 10000 天 (~27年) |

### 目录结构

```
build/android/keystore/
├── .gitignore              # 忽略密钥和密码文件
├── README.md               # 使用说明
├── keystore.env.template   # 配置模板 (可提交)
├── keystore.env            # 实际配置 (不提交,包含密码)
├── debug.keystore          # Debug 密钥 (可选提交)
└── release.keystore        # Release 密钥 (不提交)
```

### 配置文件说明 (keystore.env)

```bash
# Release 密钥配置
RELEASE_KEYSTORE_PATH="build/android/keystore/release.keystore"
RELEASE_KEYSTORE_PASSWORD="your_secure_password"     # 密钥库密码
RELEASE_KEY_ALIAS="torn_trade"                       # 密钥别名
RELEASE_KEY_PASSWORD="your_secure_password"          # 密钥密码

# Release 密钥信息 (可选,用于完全自动化生成)
RELEASE_KEY_CN="Your Name"                           # 姓名或组织名称
RELEASE_KEY_OU="Development"                         # 组织单位
RELEASE_KEY_O="Your Organization"                    # 组织
RELEASE_KEY_L="City"                                 # 城市
RELEASE_KEY_ST="State"                               # 州/省
RELEASE_KEY_C="US"                                   # 国家代码 (2位,如 CN, US)
```

**注意**:
- 如果配置了完整的密钥信息,生成密钥时将完全自动化,无需任何交互
- 如果只配置了密码,生成时会提示输入密钥信息
- 密钥信息可以留空或使用任意值,不影响应用功能

### 打包脚本工作流程

1. **读取配置**: 从 `keystore.env` 加载密钥配置
2. **选择密钥**: 根据构建类型 (debug/release) 选择对应的密钥
3. **验证密钥**:
   - Debug: 如果不存在则自动生成
   - Release: 如果不存在或密码未配置则报错
4. **配置环境变量**: 设置 `CARGO_APK_RELEASE_KEYSTORE` 等
5. **执行构建**: 调用 `cargo apk build`
6. **输出结果**: 显示 APK 路径和安装命令

---

## 常见问题

### Q1: Debug 和 Release 密钥有什么区别?

**Debug 密钥**:
- 使用固定的通用密码 (`android`)
- 主要用于开发期间快速测试
- 所有开发者可以共享同一个 Debug 密钥
- Google Play 不接受使用 Debug 密钥签名的应用

**Release 密钥**:
- 使用自定义的安全密码
- 用于发布到应用商店或分发给用户
- 必须妥善保管,丢失后无法更新已发布的应用
- 每个应用应该有唯一的 Release 密钥

### Q2: 为什么要区分两种密钥?

1. **安全性**: Release 密钥的密码更安全,不会泄露
2. **便利性**: Debug 密钥使用固定密码,开发时无需每次输入
3. **版本控制**: Debug 密钥可以提交到 git 方便团队协作,Release 密钥不能
4. **应用更新**: 同一个 Release 密钥用于应用的所有版本,确保可以更新

### Q3: 如果只是个人开发,是否可以只用一个密钥?

可以,但**不推荐**:
- 如果将 Release 密钥提交到 git,存在安全风险
- 如果只用 Debug 密钥,无法发布到应用商店

**推荐做法**:
- 开发测试使用 Debug 密钥
- 发布前使用 Release 密钥重新打包

### Q4: keystore.env 文件必须创建吗?

**Debug 构建**: 不需要,脚本会使用默认配置并自动生成 Debug 密钥

**Release 构建**: 需要,因为必须配置 Release 密钥的密码

### Q5: 密钥丢失了怎么办?

**Debug 密钥**: 无所谓,可以重新生成,不影响已安装的应用

**Release 密钥**:
- 如果已发布到应用商店: **无法恢复**,无法更新应用
- 只能用新密钥重新发布 (相当于新应用,用户数据不保留)
- **建议**: 定期备份 Release 密钥到安全位置

### Q6: 密码忘记了怎么办?

**Debug 密钥**: 密码固定为 `android`,不会忘记

**Release 密钥**:
- 无法恢复密码
- 只能重新生成新的 Release 密钥
- 如果已发布应用,会导致无法更新

---

## 安全最佳实践

### ✅ 应该做的

1. **备份 Release 密钥**: 将 `release.keystore` 备份到安全的离线位置
2. **使用强密码**: Release 密钥密码应该至少 8 位,包含字母、数字、符号
3. **不提交到 git**: 确保 `release.keystore` 和 `keystore.env` 在 `.gitignore` 中
4. **记录密码**: 将 Release 密钥密码记录在安全的密码管理器中
5. **限制访问**: 只有需要发布应用的人才能访问 Release 密钥

### ❌ 不应该做的

1. **不要共享 Release 密钥**: 不要通过聊天工具、邮件等发送
2. **不要使用弱密码**: 不要使用 `123456`, `password` 等简单密码
3. **不要提交到 git**: 即使是私有仓库也不要提交 Release 密钥
4. **不要在多个应用间共享**: 每个应用应该有独立的 Release 密钥
5. **不要存储在公共位置**: 不要放在网盘、代码托管平台等

---

## 手动操作 (高级)

如果不想使用脚本,也可以手动操作:

### 生成 Debug 密钥

```bash
keytool -genkey -v \
  -keystore build/android/keystore/debug.keystore \
  -alias androiddebugkey \
  -keyalg RSA -keysize 2048 -validity 10000 \
  -storepass android \
  -keypass android \
  -dname "CN=Android Debug,O=Android,C=US"
```

### 生成 Release 密钥

```bash
keytool -genkey -v \
  -keystore build/android/keystore/release.keystore \
  -alias torn_trade \
  -keyalg RSA -keysize 2048 -validity 10000
```

按提示输入密码和信息。

### 查看密钥信息

```bash
# 查看 Debug 密钥
keytool -list -v \
  -keystore build/android/keystore/debug.keystore \
  -storepass android

# 查看 Release 密钥 (需要输入密码)
keytool -list -v \
  -keystore build/android/keystore/release.keystore
```

---

## CI/CD 集成

如果需要在 CI/CD 中构建 Release APK,可以使用环境变量:

```bash
# 在 CI 环境中设置环境变量
export RELEASE_KEYSTORE_PASSWORD="密码从 CI secrets 读取"
export RELEASE_KEY_PASSWORD="密码从 CI secrets 读取"

# 将 base64 编码的 keystore 解码
echo "$RELEASE_KEYSTORE_BASE64" | base64 -d > build/android/keystore/release.keystore

# 运行构建
./build/package/package.sh -p android -r release
```

---

## 总结

- **开发测试**: 使用 Debug 密钥,自动生成,无需配置
- **生产发布**: 使用 Release 密钥,需要生成并配置密码
- **密钥管理**: Release 密钥和密码不要提交到 git
- **安全第一**: 妥善保管 Release 密钥,定期备份
