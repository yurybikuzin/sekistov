
# https://stackoverflow.com/questions/7252189/suppress-all-make-output-except-for-errors-and-warnings
# .SILENT:

server:
	cargo run --release -- \
		server

# wasm:
# 	which zig
# 	rm -f zig/*.wasm
# 	rm -f zig/*.o
# 	rm -f zig/*.a
# 	# -O ReleaseFast
# 	cd zig && zig build-lib lib.zig -target wasm32-freestanding -O ReleaseSmall 
# 	# -dynamic \
# 	# -rdynamic # https://dev.to/sleibrock/webassembly-with-zig-part-1-4onm#comment-2a20m
# 	ls -lAh zig/*.wasm

wasm:
	which zig
	rm -f zig/*.wasm
	# -O ReleaseFast
	cd zig && zig build-lib lib.zig -target wasm32-freestanding -O ReleaseSmall \
	-dynamic \
	-rdynamic # https://dev.to/sleibrock/webassembly-with-zig-part-1-4onm#comment-2a20m
	ls -lAh zig/*.wasm
