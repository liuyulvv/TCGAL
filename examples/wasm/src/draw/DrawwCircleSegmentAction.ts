import { KonvaEventObject } from "konva/lib/Node";
import { Vector2d } from "konva/lib/types";
import { useStageStore } from "../status/Stage";
import { DrawAction, DrawActionType } from "./DrawAction";

export class DrawCircleSegmentAction extends DrawAction {
    private static instance: DrawCircleSegmentAction;

    private drawFlag: boolean;
    private center: Vector2d | null;
    private radius: number | null;

    private constructor() {
        super();
        this.drawFlag = false;
        this.center = null;
        this.radius = null;
    }

    public static getInstance(): DrawCircleSegmentAction {
        if (!DrawCircleSegmentAction.instance) {
            DrawCircleSegmentAction.instance = new DrawCircleSegmentAction();
        }
        return DrawCircleSegmentAction.instance;
    }

    public actionType(): DrawActionType {
        return DrawActionType.CircleSegment;
    }

    public onStart(): void {}

    public onDraw(): void {}

    public onStop(): void {}

    public onPointerDown(event: KonvaEventObject<MouseEvent>): void {
        if (event.evt.button == 0) {
            const stage = event.target.getStage();
            if (stage) {
                const point = stage.getPointerPosition();
                if (point) {
                    if (this.drawFlag) {
                        useStageStore.getState().addCircle([this.center!, this.radius!]);
                        useStageStore.getState().setTempCircle(null);
                        this.drawFlag = false;
                    } else {
                        this.center = point;
                        this.drawFlag = true;
                    }
                }
            }
        } else if (event.evt.button == 2) {
            useStageStore.getState().setTempCircle(null);
            this.center = null;
            this.radius = null;
            this.drawFlag = false;
        }
    }

    public onPointerMove(event: KonvaEventObject<MouseEvent>): void {
        if (this.drawFlag) {
            const stage = event.target.getStage();
            if (stage) {
                const point = stage.getPointerPosition();
                if (point) {
                    this.radius = Math.sqrt(
                        Math.pow(this.center!.x - point.x, 2) + Math.pow(this.center!.y - point.y, 2)
                    );
                    useStageStore.getState().setTempCircle([this.center!, this.radius]);
                }
            }
        }
    }

    public onPointerUp(event: KonvaEventObject<MouseEvent>): void {}
}
