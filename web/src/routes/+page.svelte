<script lang="ts">
	import init, { World } from 'wasm-test';
	import { onMount, onDestroy } from 'svelte';

	const CELL_SIZE = 48;

	let ctx: CanvasRenderingContext2D | null;
	let canvas: HTMLCanvasElement | null;

	let world: World | null;

	let interval: number;

	onMount(() => {
		init().then((_) => {
			world = World.new();

			canvas = document.getElementById('game-canvas') as HTMLCanvasElement;
			ctx = canvas.getContext('2d');

			canvas.height = world.width() * CELL_SIZE;
			canvas.width = world.width() * CELL_SIZE;

			// Update loop
			interval = setInterval(() => {
				if (ctx === null || world === null) return;

				ctx!.clearRect(0, 0, canvas!.width, canvas!.height);

				drawWorld();
				drawSnake();

				world.update();

				console.log('Updating');
			}, 100);
		});
	});

	onDestroy(() => {
		// cleanup
		clearInterval(interval);
	});

	const drawWorld = () => {
		if (ctx === null || world === null) return;

		ctx.beginPath();

		for (let x = 0; x < world.width() + 1; ++x) {
			ctx.moveTo(CELL_SIZE * x, 0);
			ctx.lineTo(CELL_SIZE * x, world.width() * CELL_SIZE);
		}

		for (let y = 0; y < world.width() + 1; ++y) {
			ctx.moveTo(0, CELL_SIZE * y);
			ctx.lineTo(world.width() * CELL_SIZE, CELL_SIZE * y);
		}

		ctx.stroke();
	};

	const drawSnake = () => {
		if (ctx === null || world === null) return;

		const snakeIdx = world?.snake_head_idx();
		const col = snakeIdx % world.width();
		const row = Math.floor(snakeIdx / world.width());

		ctx.beginPath();

		ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);

		ctx.stroke();
	};
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
