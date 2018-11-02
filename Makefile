all:
	@cd computor_v2 && cargo build --release
	@ln -f computor_v2/target/release/computor_v2 ComputorV2
	@ls -lh ComputorV2

clean:
	@cd computor_v2 && cargo clean

fclean:
	@cd computor_v2 && cargo clean
	rm ComputorV2

re:
	@cd computor_v2 && cargo clean
	@cd computor_v2 && cargo build --release
	@ln -f computor_v2/target/release/computor_v2 ComputorV2
	@ls -lh ComputorV2
