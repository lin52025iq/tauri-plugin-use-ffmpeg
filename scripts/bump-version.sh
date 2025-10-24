#!/bin/bash

# 版本更新脚本
# 用法: ./scripts/bump-version.sh 0.1.1

if [ $# -eq 0 ]; then
    echo "用法: $0 <version>"
    echo "例如: $0 0.1.1"
    exit 1
fi

VERSION=$1

echo "更新版本号到 $VERSION..."

# 更新 Cargo.toml
if [ -f "Cargo.toml" ]; then
    sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
    rm Cargo.toml.bak
    echo "✅ 已更新 Cargo.toml"
else
    echo "❌ 未找到 Cargo.toml"
fi

# 更新 package.json
if [ -f "package.json" ]; then
    npm version $VERSION --no-git-tag-version
    echo "✅ 已更新 package.json"
else
    echo "❌ 未找到 package.json"
fi

echo ""
echo "🎉 版本号已更新到 $VERSION"
echo ""
echo "下一步："
echo "1. 检查更改: git diff"
echo "2. 提交更改: git add . && git commit -m \"Bump version to $VERSION\""
echo "3. 推送代码: git push"
echo "4. 创建标签: git tag v$VERSION"
echo "5. 推送标签: git push origin v$VERSION"
