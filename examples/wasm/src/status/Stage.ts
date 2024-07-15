import { Vector2d } from "konva/lib/types";
import { create } from "zustand";

interface StageInterface {
    lines: Array<[Vector2d, Vector2d]>;
    tempLine: [Vector2d, Vector2d] | null;
    addLine: (line: [Vector2d, Vector2d]) => void;
    addTempLine: () => void;
    setTempLine: (line: [Vector2d, Vector2d] | null) => void;
}

const useStageStore = create<StageInterface>()((set) => ({
    lines: [],
    tempLine: null,
    addLine: (line: [Vector2d, Vector2d]) => {
        set((state) => ({ lines: [...state.lines, line] }));
    },
    addTempLine: () =>
        set((state) => {
            if (state.tempLine) {
                state.lines.push(state.tempLine);
                return { lines: state.lines, tempLine: null };
            }
            return state;
        }),
    setTempLine: (line: [Vector2d, Vector2d] | null) => set({ tempLine: line }),
}));

export { useStageStore };
