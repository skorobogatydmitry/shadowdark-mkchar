VERSION = `cargo metadata --format-version 1 --no-deps | jq -r '.packages[0].version'`
HEAD_TAG = `git tag --list --points-at HEAD`
BASENAME = shadowdark-mkchar-${VERSION}

release-linux: pre-release
	mkdir -p release/linux-aarch64/${BASENAME}
	cross build --release --target aarch64-unknown-linux-gnu
	cp ./target/aarch64-unknown-linux-gnu/release/shadowdark-mkchar ./release/linux-aarch64/${BASENAME}/
	cd "./release/linux-aarch64/${BASENAME}" && sha256sum ./shadowdark-mkchar > ./shadowdark-mkchar.sha256
	cp -r lang release/linux-aarch64/${BASENAME}/
	tar -czf release/${BASENAME}-linux-aarch64.tgz -C release/linux-aarch64 ${BASENAME}

	mkdir -p release/linux-x86_64/${BASENAME}
	cross build --release --target x86_64-unknown-linux-gnu
	cp ./target/x86_64-unknown-linux-gnu/release/shadowdark-mkchar ./release/linux-x86_64/${BASENAME}/
	cd "./release/linux-x86_64/${BASENAME}" && sha256sum ./shadowdark-mkchar > ./shadowdark-mkchar.sha25
	cp -r lang release/linux-x86_64/${BASENAME}/
	tar -czf release/${BASENAME}-linux-x86_64.tgz -C release/linux-x86_64 ${BASENAME}

release-mac: pre-release
	mkdir -p release/darwin-aarch64/${BASENAME}
	cross build --release --target aarch64-apple-darwin
	cp ./target/aarch64-apple-darwin/release/shadowdark-mkchar ./release/darwin-aarch64/${BASENAME}/
	cd "./release/darwin-aarch64/${BASENAME}" && shasum -a 256 ./shadowdark-mkchar > ./shadowdark-mkchar.sha256
	cp -r lang release/darwin-aarch64/${BASENAME}/
	tar -czf release/${BASENAME}-darwin-aarch64.tgz -C release/darwin-aarch64 ${BASENAME}

	mkdir -p release/darwin-x86_64/${BASENAME}
	cross build --release --target x86_64-apple-darwin
	cp ./target/x86_64-apple-darwin/release/shadowdark-mkchar ./release/darwin-x86_64/${BASENAME}/
	cd "./release/darwin-x86_64/${BASENAME}" && shasum -a 256 ./shadowdark-mkchar > ./shadowdark-mkchar.sha256
	cp -r lang release/darwin-x86_64/${BASENAME}/
	tar -czf release/${BASENAME}-darwin-x86_64.tgz -C release/darwin-x86_64 ${BASENAME}

release-win: pre-release
	mkdir -p release/windows-aarch64/${BASENAME}
	cross build --release --target aarch64-pc-windows-msvc
	cp ./target/aarch64-pc-windows-msvc/release/shadowdark-mkchar ./release/windows-aarch64/${BASENAME}/
	cd "./release/windows-aarch64/${BASENAME}" && sha256sum ./shadowdark-mkchar > ./shadowdark-mkchar.sha256
	cp -r lang release/windows-aarch64/${BASENAME}/
	tar -czf release/${BASENAME}-windows-aarch64.tgz -C release/windows-aarch64 ${BASENAME}

	mkdir -p release/windows-x86_64/${BASENAME}
	cross build --release --target x86_64-pc-windows-msvc
	cp ./target/x86_64-pc-windows-msvc/release/shadowdark-mkchar ./release/windows-x86_64/${BASENAME}/
	cd "./release/windows-x86_64/${BASENAME}" && sha256sum ./shadowdark-mkchar > ./shadowdark-mkchar.sha256
	cp -r lang release/windows-x86_64/${BASENAME}/
	tar -czf release/${BASENAME}-windows-x86_64.tgz -C release/windows-x86_64 ${BASENAME}

pre-release:
	jq -V
	sha256sum --version || shasum --version

	rm -rf ./release

	git diff HEAD --quiet
	test -z ${HEAD_TAG} && git tag ${VERSION} || true
	test ${HEAD_TAG} == ${VERSION}
	git push --tags
