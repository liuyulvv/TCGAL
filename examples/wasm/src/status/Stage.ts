import { Vector2d } from "konva/lib/types";
import { CircleSegment2, LineSegment2, Point2, Segment2, Segment2Type, SweepSegment2Intersection } from "rcgal";
import { create } from "zustand";

interface StageInterface {
    cost: number | null;
    points: Array<[Vector2d, number]>;
    lines: Array<[Vector2d, Vector2d]>;
    circles: Array<[Vector2d, number]>;
    tempLine: [Vector2d, Vector2d] | null;
    tempCircle: [Vector2d, number] | null;
    clear: () => void;
    addLine: (line: [Vector2d, Vector2d]) => void;
    addTempLine: () => void;
    setTempLine: (line: [Vector2d, Vector2d] | null) => void;
    addCircle: (circle: [Vector2d, number]) => void;
    addTempCircle: () => void;
    setTempCircle: (circle: [Vector2d, number] | null) => void;
}

const useStageStore = create<StageInterface>()((set) => ({
    cost: null,
    points: [],
    lines: [],
    circles: [],
    tempLine: null,
    tempCircle: null,
    clear: () => set({ cost: null, points: [], lines: [], circles: [], tempLine: null, tempCircle: null }),
    addLine: (line: [Vector2d, Vector2d]) =>
        set((state) => {
            const updateLines = [...state.lines, line];
            const circles = state.circles;
            const intersection = intersect(updateLines, circles);
            return { lines: updateLines, ...intersection };
        }),
    addTempLine: () =>
        set((state) => {
            if (state.tempLine) {
                const updateLines = [...state.lines, state.tempLine];
                const circles = state.circles;
                const intersection = intersect(updateLines, circles);
                return { lines: updateLines, tempLine: null, ...intersection };
            }
            return state;
        }),
    setTempLine: (line: [Vector2d, Vector2d] | null) => set({ tempLine: line }),
    addCircle: (circle: [Vector2d, number]) =>
        set((state) => {
            const updateCircles = [...state.circles, circle];
            const lines = state.lines;
            const intersection = intersect(lines, updateCircles);
            return { circles: updateCircles, ...intersection };
        }),
    addTempCircle: () =>
        set((state) => {
            if (state.tempCircle) {
                const updateCircles = [...state.circles, state.tempCircle];
                const lines = state.lines;
                const intersection = intersect(lines, updateCircles);
                return { circles: updateCircles, tempCircle: null, ...intersection };
            }
            return state;
        }),
    setTempCircle: (circle: [Vector2d, number] | null) => set({ tempCircle: circle }),
}));

const intersect = (lines: Array<[Vector2d, Vector2d]>, circles: Array<[Vector2d, number]>) => {
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
    circles.forEach((circle) => {
        const [center, radius] = circle;
        intersection.push_segment(
            new Segment2(
                Segment2Type.CircleSegment2,
                undefined,
                new CircleSegment2(new Point2(center.x, center.y), radius)
            )
        );
    });
    const start = Date.now();
    const points = intersection.intersection();
    const end = Date.now();
    let result: Array<[Vector2d, number]> = [];
    points.forEach((point) => {
        result.push([{ x: point.x(), y: point.y() }, 5]);
    });
    return { points: result, cost: end - start };
};

export { useStageStore };
