#!/bin/bash

# Android 签名密钥生成脚本

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
KEYSTORE_DIR="$PROJECT_ROOT/build/android/keystore"

# 颜色输出
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Android 签名密钥生成工具 ===${NC}\n"

# 确保目录存在
mkdir -p "$KEYSTORE_DIR"

# 显示菜单
echo "请选择要生成的密钥类型:"
echo "  1) Debug 密钥 (用于开发调试,使用默认密码)"
echo "  2) Release 密钥 (用于生产发布,需要设置密码)"
echo "  3) 两者都生成"
echo ""
read -p "请选择 [1-3]: " choice

generate_debug_key() {
    local keystore_path="$KEYSTORE_DIR/debug.keystore"

    if [ -f "$keystore_path" ]; then
        echo -e "${YELLOW}Debug 密钥已存在: $keystore_path${NC}"
        read -p "是否覆盖? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "跳过 Debug 密钥生成"
            return
        fi
        rm -f "$keystore_path"
    fi

    echo -e "${GREEN}生成 Debug 密钥...${NC}"
    keytool -genkey -v \
        -keystore "$keystore_path" \
        -alias androiddebugkey \
        -keyalg RSA \
        -keysize 2048 \
        -validity 10000 \
        -storepass android \
        -keypass android \
        -dname "CN=Android Debug,O=Android,C=US"

    echo -e "${GREEN}✓ Debug 密钥生成完成${NC}"
    echo -e "  路径: $keystore_path"
    echo -e "  别名: androiddebugkey"
    echo -e "  密码: android / android"
    echo ""
}

generate_release_key() {
    local keystore_path="$KEYSTORE_DIR/release.keystore"
    local keystore_env="$KEYSTORE_DIR/keystore.env"

    if [ -f "$keystore_path" ]; then
        echo -e "${YELLOW}Release 密钥已存在: $keystore_path${NC}"
        read -p "是否覆盖? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "跳过 Release 密钥生成"
            return
        fi
        rm -f "$keystore_path"
    fi

    echo -e "${GREEN}生成 Release 密钥...${NC}"
    echo ""

    # 尝试从配置文件读取密码
    local store_password=""
    local key_password=""
    local dname=""

    if [ -f "$keystore_env" ]; then
        # shellcheck source=/dev/null
        source "$keystore_env"
        store_password="${RELEASE_KEYSTORE_PASSWORD:-}"
        key_password="${RELEASE_KEY_PASSWORD:-}"

        # 检查密码是否有效(不是占位符)
        if [[ -n "$store_password" && "$store_password" != "your_store_password_here" && "$store_password" != "your_password_here" ]]; then
            echo -e "${GREEN}✓ 从配置文件读取到 Release 密钥密码${NC}"
            echo -e "  配置文件: $keystore_env"

            # 尝试读取密钥信息
            local cn="${RELEASE_KEY_CN:-}"
            local ou="${RELEASE_KEY_OU:-}"
            local o="${RELEASE_KEY_O:-}"
            local l="${RELEASE_KEY_L:-}"
            local st="${RELEASE_KEY_ST:-}"
            local c="${RELEASE_KEY_C:-}"

            # 构建 dname (只包含非空字段)
            if [[ -n "$cn" && "$cn" != "Your Name" ]]; then
                dname="CN=$cn"
                [[ -n "$ou" && "$ou" != "Development" ]] && dname="$dname, OU=$ou"
                [[ -n "$o" && "$o" != "Your Organization" ]] && dname="$dname, O=$o"
                [[ -n "$l" && "$l" != "City" ]] && dname="$dname, L=$l"
                [[ -n "$st" && "$st" != "State" ]] && dname="$dname, ST=$st"
                [[ -n "$c" && "$c" != "US" ]] && dname="$dname, C=$c"
                echo -e "${GREEN}✓ 从配置文件读取到密钥信息${NC}"
            fi
        else
            store_password=""
            key_password=""
        fi
    fi

    # 如果配置文件中没有有效密码,则提示用户输入
    if [[ -z "$store_password" ]]; then
        echo -e "${YELLOW}未找到有效的密码配置,请手动输入:${NC}"
        while [[ -z "$store_password" ]]; do
            read -sp "请输入密钥库密码 (storepass): " store_password
            echo
            if [[ -z "$store_password" ]]; then
                echo -e "${RED}密码不能为空,请重新输入${NC}"
            fi
        done

        while [[ -z "$key_password" ]]; do
            read -sp "请输入密钥密码 (keypass,建议与密钥库密码相同): " key_password
            echo
            if [[ -z "$key_password" ]]; then
                echo -e "${RED}密码不能为空,请重新输入${NC}"
            fi
        done

        # 保存到配置文件
        local should_save=true
        if [ -f "$keystore_env" ]; then
            echo ""
            read -p "是否保存密码到配置文件? (Y/n): " -n 1 -r
            echo
            if [[ $REPLY =~ ^[Nn]$ ]]; then
                should_save=false
            fi
        fi

        if [ "$should_save" = true ]; then
            # 创建或更新配置文件
            if [ ! -f "$keystore_env" ]; then
                if [ -f "$KEYSTORE_DIR/keystore.env.template" ]; then
                    cp "$KEYSTORE_DIR/keystore.env.template" "$keystore_env"
                    echo -e "${GREEN}✓ 已从模板创建配置文件${NC}"
                else
                    cat > "$keystore_env" << 'EOF'
# Android 打包签名配置

# Debug 密钥配置 (默认值,可以公开)
DEBUG_KEYSTORE_PATH="build/android/keystore/debug.keystore"
DEBUG_KEYSTORE_PASSWORD="android"
DEBUG_KEY_ALIAS="androiddebugkey"
DEBUG_KEY_PASSWORD="android"

# Release 密钥配置 (请修改为实际密码)
RELEASE_KEYSTORE_PATH="build/android/keystore/release.keystore"
RELEASE_KEYSTORE_PASSWORD="your_store_password_here"
RELEASE_KEY_ALIAS="torn_trade"
RELEASE_KEY_PASSWORD="your_key_password_here"

# Release 密钥信息 (可选,用于自动生成密钥)
RELEASE_KEY_CN="Your Name"
RELEASE_KEY_OU="Development"
RELEASE_KEY_O="Your Organization"
RELEASE_KEY_L="City"
RELEASE_KEY_ST="State"
RELEASE_KEY_C="US"
EOF
                    echo -e "${GREEN}✓ 已创建配置文件${NC}"
                fi
            fi

            # 更新密码
            if command -v sed >/dev/null 2>&1; then
                if [[ "$OSTYPE" == "darwin"* ]]; then
                    sed -i '' "s|^RELEASE_KEYSTORE_PASSWORD=.*|RELEASE_KEYSTORE_PASSWORD=\"$store_password\"|" "$keystore_env"
                    sed -i '' "s|^RELEASE_KEY_PASSWORD=.*|RELEASE_KEY_PASSWORD=\"$key_password\"|" "$keystore_env"
                else
                    sed -i "s|^RELEASE_KEYSTORE_PASSWORD=.*|RELEASE_KEYSTORE_PASSWORD=\"$store_password\"|" "$keystore_env"
                    sed -i "s|^RELEASE_KEY_PASSWORD=.*|RELEASE_KEY_PASSWORD=\"$key_password\"|" "$keystore_env"
                fi
                echo -e "${GREEN}✓ 密码已保存到: $keystore_env${NC}"
            else
                echo -e "${YELLOW}⚠ sed 命令不可用,请手动编辑配置文件${NC}"
            fi
        fi
    fi

    echo ""

    # 生成密钥
    if [[ -n "$dname" ]]; then
        # 使用配置的 dname,无需交互
        echo -e "${GREEN}使用配置的密钥信息自动生成...${NC}"
        keytool -genkey -v \
            -keystore "$keystore_path" \
            -alias torn_trade \
            -keyalg RSA \
            -keysize 2048 \
            -validity 10000 \
            -storepass "$store_password" \
            -keypass "$key_password" \
            -dname "$dname"
    else
        # 没有配置 dname,需要交互式输入
        echo -e "${YELLOW}请输入密钥信息 (姓名、组织等,可留空):${NC}"
        keytool -genkey -v \
            -keystore "$keystore_path" \
            -alias torn_trade \
            -keyalg RSA \
            -keysize 2048 \
            -validity 10000 \
            -storepass "$store_password" \
            -keypass "$key_password"
    fi

    echo ""
    echo -e "${GREEN}✓ Release 密钥生成完成${NC}"
    echo -e "  路径: $keystore_path"
    echo -e "  别名: torn_trade"
    echo ""
}

# 根据选择生成密钥
case $choice in
    1)
        generate_debug_key
        ;;
    2)
        generate_release_key
        ;;
    3)
        generate_debug_key
        generate_release_key
        ;;
    *)
        echo -e "${RED}无效的选择${NC}"
        exit 1
        ;;
esac

echo -e "${BLUE}=== 完成 ===${NC}"
echo "密钥文件已生成在: $KEYSTORE_DIR"
echo ""
echo "后续步骤:"
echo "  运行 Android 打包脚本进行测试"
echo "  ./build/package/package.sh -p android -r debug"
echo "  ./build/package/package.sh -p android -r release"


