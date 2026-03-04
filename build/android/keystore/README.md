# Android Keystore 目录

此目录用于存放 Android 应用签名的密钥库文件。

## 快速开始

1. **生成密钥**:
   ```bash
   ./scripts/setup_android_signing.sh
   ```

   选择选项 2 或 3 时,会自动配置 Release 密钥密码到 `keystore.env`

2. **构建应用**:
   ```bash
   # Debug 构建
   ./build/package/package.sh -p android -r debug

   # Release 构建
   ./build/package/package.sh -p android -r release
   ```

## 文件说明

| 文件 | 说明 | Git |
|------|------|-----|
| `debug.keystore` | 开发调试密钥 (密码: android) | 可选提交 |
| `release.keystore` | 生产发布密钥 (自定义密码) | **不提交** |
| `keystore.env` | 密钥配置 (包含密码) | **不提交** |
| `keystore.env.template` | 配置模板 | 可提交 |

## 密钥类型

### Debug 密钥
- **用途**: 开发测试
- **密码**: 固定为 `android`
- **自动生成**: 首次构建时自动创建
- **安全性**: 低 (通用密码)

### Release 密钥
- **用途**: 生产发布
- **密码**: 自定义 (需保密)
- **手动生成**: 运行 `setup_android_signing.sh`
- **安全性**: 高 (需妥善保管)

## 重要提示

⚠️ **Release 密钥安全**:
- 不要提交到 git
- 不要分享给他人
- 定期备份到安全位置
- 使用强密码
- 密钥丢失将无法更新已发布的应用

📖 **详细文档**: 查看 [Android 签名配置指南](../../../docs/ANDROID_SIGNING_GUIDE.md)

## 手动生成密钥

### Debug 密钥
```bash
keytool -genkey -v \
  -keystore debug.keystore \
  -alias androiddebugkey \
  -keyalg RSA -keysize 2048 -validity 10000 \
  -storepass android \
  -keypass android \
  -dname "CN=Android Debug,O=Android,C=US"
```

### Release 密钥
```bash
keytool -genkey -v \
  -keystore release.keystore \
  -alias torn_trade \
  -keyalg RSA -keysize 2048 -validity 10000
```

## 查看密钥信息

```bash
# Debug 密钥
keytool -list -v -keystore debug.keystore -storepass android

# Release 密钥 (需要输入密码)
keytool -list -v -keystore release.keystore
```
