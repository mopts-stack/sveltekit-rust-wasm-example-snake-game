<script lang="ts">
	import { onMount } from 'svelte';

	import init, { greet, type InitOutput } from 'wasm-test';

	let sum: WebAssembly.ExportValue;
	let mem: WebAssembly.Memory;
	let wasm_test: InitOutput;

	async function start() {
		// const byteArray = new Int8Array([
		// 	0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x07, 0x01, 0x60, 0x02, 0x7f, 0x7f,
		// 	0x01, 0x7f, 0x03, 0x02, 0x01, 0x00, 0x07, 0x07, 0x01, 0x03, 0x73, 0x75, 0x6d, 0x00, 0x00,
		// 	0x0a, 0x09, 0x01, 0x07, 0x00, 0x20, 0x00, 0x20, 0x01, 0x6a, 0x0b, 0x00, 0x18, 0x04, 0x6e,
		// 	0x61, 0x6d, 0x65, 0x01, 0x06, 0x01, 0x00, 0x03, 0x73, 0x75, 0x6d, 0x02, 0x09, 0x01, 0x00,
		// 	0x02, 0x00, 0x01, 0x61, 0x01, 0x01, 0x62
		// ]);

		const memory = new WebAssembly.Memory({
			initial: 1
		});

		const importObject = {
			js: {
				memory
			},
			console: {
				log: () => {
					console.log('Just logging something');
				},
				error: () => {
					console.log('I an just error');
				}
			}
		};

		const response = await fetch('/wasm/sum.wasm');
		const buffer = await response.arrayBuffer();

		const wasm = await WebAssembly.instantiate(buffer, importObject);
		sum = wasm.instance.exports.sum;
		mem = wasm.instance.exports.mem as WebAssembly.Memory;

		// actual wasm generated from rust
		wasm_test = await init(); // for demonstration purposes, we can just run the init()
	}

	onMount(async () => {
		await start();
		// @ts-ignore
		const result = sum(100, 1000);
		console.log(result);

		const uint8Array = new Uint8Array(mem.buffer, 0, 2);
		const text = new TextDecoder().decode(uint8Array);

		console.log(text);

		greet('navid'); // we could use wasm_test.greet instead
	});
</script>
