import { KonvaEventObject } from "konva/lib/Node";
import { useStageStoreVanilla } from "../status/Stage";
import { DrawAction, DrawActionType } from "./DrawAction";

export class DrawLineSegmentAction extends DrawAction {
    private static instance: DrawLineSegmentAction;

    private drawFlag: boolean;

    private constructor() {
        super();
        this.drawFlag = false;
    }

    public static getInstance(): DrawLineSegmentAction {
        if (!DrawLineSegmentAction.instance) {
            DrawLineSegmentAction.instance = new DrawLineSegmentAction();
        }
        return DrawLineSegmentAction.instance;
    }

    public actionType(): DrawActionType {
        return DrawActionType.LineSegment;
    }

    public onStart(): void {}

    public onDraw(): void {}

    public onStop(): void {}

    public onPointerDown(event: KonvaEventObject<MouseEvent>): void {
        if (event.evt.button == 0) {
            const stage = event.target.getStage();
            if (stage) {
                console.log(event.target.getStage()?.getPointerPosition());
                if (this.drawFlag) {
                    const lines = useStageStoreVanilla.getState().lines;
                    const tempLine = useStageStoreVanilla.getState().tempLine;
                    if (tempLine) {
                        useStageStoreVanilla.getState().addLine(tempLine);
                        // useStageStoreVanilla.setState({
                        //     lines: [...lines, tempLine],
                        //     tempLine: null,
                        // });
                    }
                    this.drawFlag = false;
                } else {
                    const start = stage.getPointerPosition();
                    const end = stage.getPointerPosition();
                    if (start && end) {
                        useStageStoreVanilla.getState().setTempLine([start, end]);
                        // useStageStoreVanilla.setState({
                        //     tempLine: [start, end],
                        // });
                        this.drawFlag = true;
                    }
                }
            }
        } else if (event.evt.button == 2) {
            this.drawFlag = false;
            useStageStoreVanilla.getState().setTempLine(null);
            // useStageStoreVanilla.setState({
            //     tempLine: null,
            // });
        }
    }

    public onPointerMove(event: KonvaEventObject<MouseEvent>): void {
        if (this.drawFlag) {
            const tempLine = useStageStoreVanilla.getState().tempLine;
            if (tempLine) {
                const stage = event.target.getStage();
                if (stage) {
                    const end = stage.getPointerPosition();
                    if (end) {
                        useStageStoreVanilla.getState().setTempLine([tempLine[0], end]);
                        // useStageStoreVanilla.setState({
                        //     tempLine: [tempLine[0], end],
                        // });
                    }
                }
            }
        }
    }

    public onPointerUp(event: KonvaEventObject<MouseEvent>): void {
        // console.log(event);
    }
}
