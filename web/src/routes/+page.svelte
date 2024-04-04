<script lang="ts">
	import init, { GameStatus, World } from 'wasm-test';
	import { onDestroy, onMount } from 'svelte';

	import { SnakeGame, CELL_SIZE, WORLD_WIDTH, SNAKE_SPAWN_IDX } from '$lib/game';
	import { gameStore } from '$lib/store';

	let game: SnakeGame;

	onMount(() => {
		gameStore.reset();
		init().then((wasm) => {
			const world = World.new(WORLD_WIDTH, SNAKE_SPAWN_IDX);

			const canvas = document.getElementById('game-canvas') as HTMLCanvasElement;
			const ctx = canvas.getContext('2d')!;

			canvas.height = world.width() * CELL_SIZE;
			canvas.width = world.width() * CELL_SIZE;

			game = new SnakeGame(wasm, ctx, canvas, world);

			// initial render
			game.render();

			// Add the event listener if in a browser environment
			if (typeof window !== 'undefined') {
				document.addEventListener('keydown', game.keyboardEvents);
			}

			gameStore.updateStatus(game);
		});
	});

	onDestroy(() => {
		// Remove the event listener if in a browser environment
		if (typeof window !== 'undefined') {
			document.removeEventListener('keydown', game.keyboardEvents);
		}
	});

	const handleButtonClick = () => {
		if (game.status() === GameStatus.Lost || game.status() === GameStatus.Won) {
			game.reset();
		} else {
			game.start();
		}
	};
</script>

<div class="content-wrapper">
	<div class="game-panel">
		<div class="flex">
			<div class="label">Status:</div>

			<div id="game-status">{$gameStore.status}</div>
		</div>

		<div class="flex">
			<button disabled={$gameStore.disableButton} on:click={handleButtonClick}>Play</button>
		</div>
	</div>
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

	.game-panel {
		margin-bottom: 20px;
	}

	.flex {
		display: flex;
	}

	.label {
		font-weight: bold;
		margin-right: 10px;
	}
</style>
