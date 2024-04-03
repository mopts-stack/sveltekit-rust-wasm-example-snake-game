import { World, Direction, type InitOutput } from 'wasm-test';

export const CELL_SIZE = 48;
export const WORLD_WIDTH = 16;
export const SNAKE_SPAWN_IDX = Date.now() % (WORLD_WIDTH * WORLD_WIDTH);

const FPS = 10;

export class SnakeGame {
    constructor(readonly wasm: InitOutput, readonly ctx: CanvasRenderingContext2D, readonly canvas: HTMLCanvasElement, readonly world: World) {
    }

    mainLoop = () => {
        setTimeout(() => {
            this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

            this.world.calculate_snake_next_cell();

            this.render();

            // invoke update again before repaint event
            requestAnimationFrame(this.mainLoop);
        }, 1000 / FPS);
    }

    keyboardEvents = (e: KeyboardEvent) => {
        switch (e.code) {
            case "ArrowUp":
                this.world.change_snake_dir(Direction.Up);
                break;

            case "ArrowDown":
                this.world.change_snake_dir(Direction.Down);
                break;

            case "ArrowRight":
                this.world.change_snake_dir(Direction.Right);
                break;

            case "ArrowLeft":
                this.world.change_snake_dir(Direction.Left);
                break;
        }
    }

    render = () => {
        this.renderWorld();
        this.renderSnake();
    }

    private renderWorld = () => {
        this.ctx.beginPath();

        for (let x = 0; x < this.world.width() + 1; ++x) {
            this.ctx.moveTo(CELL_SIZE * x, 0);
            this.ctx.lineTo(CELL_SIZE * x, this.world.width() * CELL_SIZE);
        }

        for (let y = 0; y < this.world.width() + 1; ++y) {
            this.ctx.moveTo(0, CELL_SIZE * y);
            this.ctx.lineTo(this.world.width() * CELL_SIZE, CELL_SIZE * y);
        }

        this.ctx.stroke();
    };

    private renderSnake = () => {
        const snakeCells = new Uint32Array(
            this.wasm.memory.buffer,
            this.world.snake_cells(),
            this.world.snake_length()
        );

        this.ctx.beginPath();

        snakeCells.forEach((cellIdx, i) => {
            const col = cellIdx % this.world.width();
            const row = Math.floor(cellIdx / this.world.width());

            this.ctx.fillStyle = i === 0 ? "#7878db" : "#000000";

            this.ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
        });

        this.ctx.stroke();
    };
}

