import { Vector2d } from "konva/lib/types";
import { LineSegment2, Point2, Segment2, Segment2Type, SweepSegment2Intersection } from "rcgal";
import { create } from "zustand";

interface StageInterface {
    points: Array<[Vector2d, number]>;
    lines: Array<[Vector2d, Vector2d]>;
    tempLine: [Vector2d, Vector2d] | null;
    addLine: (line: [Vector2d, Vector2d]) => void;
    addTempLine: () => void;
    setTempLine: (line: [Vector2d, Vector2d] | null) => void;
}

const useStageStore = create<StageInterface>()((set) => ({
    points: [],
    lines: [],
    tempLine: null,
    addLine: (line: [Vector2d, Vector2d]) =>
        set((state) => {
            const updateLines = [...state.lines, line];
            const updatePoints = intersect(updateLines);
            return { lines: updateLines, points: updatePoints };
        }),
    addTempLine: () =>
        set((state) => {
            if (state.tempLine) {
                const updateLines = [...state.lines, state.tempLine];
                const updatePoints = intersect(updateLines);
                return { lines: updateLines, tempLine: null, points: updatePoints };
            }
            return state;
        }),
    setTempLine: (line: [Vector2d, Vector2d] | null) => set({ tempLine: line }),
}));

const intersect = (lines: Array<[Vector2d, Vector2d]>): Array<[Vector2d, number]> => {
    const intersection = new SweepSegment2Intersection();
    lines.forEach((line) => {
        const [start, end] = line;
        intersection.push_segment(
            new Segment2(
                Segment2Type.LineSegment2,
                new LineSegment2(new Point2(start.x, start.y), new Point2(end.x, end.y))
            )
        );
    });
    const points = intersection.intersection();
    let result: Array<[Vector2d, number]> = [];
    points.forEach((point) => {
        result.push([{ x: point.x(), y: point.y() }, 5]);
    });
    return result;
};

export { useStageStore };
