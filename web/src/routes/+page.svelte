<script lang="ts">
	import init, { World } from 'wasm-test';
	import { onDestroy, onMount } from 'svelte';

	import { update, draw, keyboardEvents, CELL_SIZE, WORLD_WIDTH, SNAKE_SPAWN_IDX } from '$lib/game';

	let ctx: CanvasRenderingContext2D | null;
	let canvas: HTMLCanvasElement | null;
	let world: World | null;

	function handleKeyDown(e: KeyboardEvent) {
		keyboardEvents(e, world);
	}

	onMount(() => {
		init().then((wasm) => {
			world = World.new(WORLD_WIDTH, SNAKE_SPAWN_IDX);

			canvas = document.getElementById('game-canvas') as HTMLCanvasElement;
			ctx = canvas.getContext('2d');

			canvas.height = world.width() * CELL_SIZE;
			canvas.width = world.width() * CELL_SIZE;

			draw(ctx, canvas, world);

			// Update loop
			update(ctx, canvas, world);

			const snakeCellPtr = world.snake_cells();
			const snakeLen = world.snake_length();

			const snakeCells = new Uint32Array(wasm.memory.buffer, snakeCellPtr, snakeLen);

			console.log(snakeCells);

			// Add the event listener if in a browser environment
			if (typeof window !== 'undefined') {
				document.addEventListener('keydown', handleKeyDown);
			}
		});
	});

	onDestroy(() => {
		// Remove the event listener if in a browser environment
		if (typeof window !== 'undefined') {
			document.removeEventListener('keydown', handleKeyDown);
		}
	});
</script>

<div class="content-wrapper">
	<canvas id="game-canvas"></canvas>
</div>

<style>
	.content-wrapper {
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		position: absolute;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-direction: column;
	}
</style>
