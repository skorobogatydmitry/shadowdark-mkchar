VERSION = `cargo metadata --format-version 1 --no-deps | jq -r '.packages[0].version'`
BASENAME = shadowdark-mkchar-${VERSION}

release-linux:
	rm -rf ./release

	git diff HEAD --quiet
	git tag ${VERSION}
	git push --tags

	mkdir -p release/linux-aarch64/${BASENAME}
	cross build --release --target aarch64-unknown-linux-gnu 
	cp ./target/aarch64-unknown-linux-gnu/release/shadowdark-mkchar ./release/linux-aarch64/${BASENAME}/
	sha256sum "./release/linux-aarch64/${BASENAME}/shadowdark-mkchar" > "./release/linux-aarch64/${BASENAME}/shadowdark-mkchar.sha256"
	cp -r lang release/linux-aarch64/${BASENAME}/
	tar -czf release/${BASENAME}-linux-aarch64.tgz -C release/linux-aarch64 ${BASENAME}

	mkdir -p release/linux-x86_64/${BASENAME}
	cross build --release --target x86_64-unknown-linux-gnu
	cp ./target/x86_64-unknown-linux-gnu/release/shadowdark-mkchar ./release/linux-x86_64/${BASENAME}/
	sha256sum ./release/linux-x86_64/${BASENAME}/shadowdark-mkchar > ./release/linux-x86_64/${BASENAME}/shadowdark-mkchar.sha256
	cp -r lang release/linux-x86_64/${BASENAME}/
	tar -czf release/${BASENAME}-linux-x86_64.tgz -C release/linux-x86_64 ${BASENAME}
