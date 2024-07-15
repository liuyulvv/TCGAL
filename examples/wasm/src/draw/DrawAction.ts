import { KonvaEventObject } from "konva/lib/Node";

export enum DrawActionType {
    None,
    LineSegment,
    CircleSegment,
    ArcSegment,
}

export abstract class DrawAction {
    public abstract actionType(): DrawActionType;
    public onStart(): void {}
    public onDraw(): void {}
    public onStop(): void {}
    public onPointerDown(_event: KonvaEventObject<MouseEvent>): void {}
    public onPointerMove(_event: KonvaEventObject<MouseEvent>): void {}
    public onPointerUp(_event: KonvaEventObject<MouseEvent>): void {}
}
