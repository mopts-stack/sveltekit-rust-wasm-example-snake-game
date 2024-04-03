import { World } from 'wasm-test';

export const CELL_SIZE = 48;
export const WORLD_WIDTH = 16;
export const SNAKE_SPAWN_IDX = Date.now() % (WORLD_WIDTH * WORLD_WIDTH);

const FPS = 7;

export const update = (ctx: CanvasRenderingContext2D | null, canvas: HTMLCanvasElement | null, world: World | null) => {
    setTimeout(() => {
        if (ctx === null || world === null || canvas === null) return;
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        world.update();

        draw(ctx, canvas, world);

        // invoke update again before repaint event
        requestAnimationFrame(() => update(ctx, canvas, world));
    }, 1000 / FPS);
}

export const draw = (ctx: CanvasRenderingContext2D | null, canvas: HTMLCanvasElement | null, world: World | null) => {
    if (ctx === null || world === null || canvas === null) return;

    drawWorld(ctx, world);
    drawSnake(ctx, world);
}

const drawWorld = (ctx: CanvasRenderingContext2D | null, world: World | null) => {
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

const drawSnake = (ctx: CanvasRenderingContext2D | null, world: World | null) => {
    if (ctx === null || world === null) return;

    const snakeIdx = world?.snake_head_idx();
    const col = snakeIdx % world.width();
    const row = Math.floor(snakeIdx / world.width());

    ctx.beginPath();

    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);

    ctx.stroke();
};