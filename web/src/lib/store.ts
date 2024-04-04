import { writable } from 'svelte/store';
import { GameStatus } from 'wasm-test';
import type { SnakeGame } from './game';

export type GameStore = {
    status: string,
    disableButton: boolean,
    points: number,
}

const stat = writable<GameStore>({
    disableButton: true,
    status: 'None',
    points: 0,
});

function createGameStatusStore() {
    const { subscribe, update, set } = stat;

    return {
        subscribe,
        reset: () => set({
            disableButton: true,
            status: 'None',
            points: 0,
        }),
        changeDisabled: (value: boolean) => update(s => {
            s.disableButton = value;
            return { ...s };
        }),
        changeStatus: (status: string) => update(s => {
            s.status = status;
            return { ...s };
        }),
        updateStatus: (game?: SnakeGame) => update(s => {
            s.status = game ? game.status_text() : 'None';

            s.disableButton = false;

            if (game && game.status() === GameStatus.Played) {
                s.disableButton = true;
            }

            s.points = game ? game.points() : 0;

            return s;
        }),
    }
}

export const gameStore = createGameStatusStore();
