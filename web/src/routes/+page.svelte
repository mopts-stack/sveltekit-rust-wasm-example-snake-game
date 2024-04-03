<script lang="ts">
	import init, { World } from 'wasm-test';
	import { onDestroy, onMount } from 'svelte';

	import { SnakeGame, CELL_SIZE, WORLD_WIDTH, SNAKE_SPAWN_IDX } from '$lib/game';

	let game: SnakeGame;

	onMount(() => {
		init().then((wasm) => {
			const world = World.new(WORLD_WIDTH, SNAKE_SPAWN_IDX);

			const canvas = document.getElementById('game-canvas') as HTMLCanvasElement;
			const ctx = canvas.getContext('2d')!;

			canvas.height = world.width() * CELL_SIZE;
			canvas.width = world.width() * CELL_SIZE;

			game = new SnakeGame(wasm, ctx, canvas, world);

			game.draw();

			// Update loop
			game.update();

			// Add the event listener if in a browser environment
			if (typeof window !== 'undefined') {
				document.addEventListener('keydown', game.keyboardEvents);
			}
		});
	});

	onDestroy(() => {
		// Remove the event listener if in a browser environment
		if (typeof window !== 'undefined') {
			document.removeEventListener('keydown', game.keyboardEvents);
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
