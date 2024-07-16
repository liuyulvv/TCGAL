import { Button, Label, Radio, RadioGroup } from "@fluentui/react-components";
import { KonvaEventObject } from "konva/lib/Node";
import { useEffect, useRef, useState } from "react";
import { Circle, Layer, Line, Stage } from "react-konva";
import { DrawActionType } from "./draw/DrawAction";
import { DrawManager } from "./draw/DrawManager";
import { useStageStore } from "./status/Stage";

function App() {
    const stageContainerRef = useRef<HTMLDivElement>(null);
    const cost = useStageStore((state) => state.cost);
    const points = useStageStore((state) => state.points);
    const lines = useStageStore((state) => state.lines);
    const circles = useStageStore((state) => state.circles);
    const tempLine = useStageStore((state) => state.tempLine);
    const tempCircle = useStageStore((state) => state.tempCircle);
    const [width, setWidth] = useState<number>(0);
    const [height, setHeight] = useState<number>(0);
    const [drawType, setDrawType] = useState(DrawActionType.None);
    const [drawManager, _setDrawManager] = useState(DrawManager.getInstance());

    useEffect(() => {
        if (stageContainerRef.current) {
            setWidth(stageContainerRef.current.clientWidth);
            setHeight(stageContainerRef.current.clientHeight);
        }
    }, [stageContainerRef, width, height]);

    const handlePointerDown = (e: KonvaEventObject<MouseEvent>) => {
        drawManager.getActiveAction()?.onPointerDown(e);
    };

    const handlePointerMove = (e: KonvaEventObject<MouseEvent>) => {
        drawManager.getActiveAction()?.onPointerMove(e);
    };

    const handlePointerUp = (e: KonvaEventObject<MouseEvent>) => {
        drawManager.getActiveAction()?.onPointerUp(e);
    };

    return (
        <div className="container">
            <div className="main">
                <RadioGroup
                    layout="horizontal"
                    onChange={(_, data) => {
                        switch (data.value) {
                            case "line-segment": {
                                setDrawType(DrawActionType.LineSegment);
                                drawManager.setActiveAction(DrawActionType.LineSegment);
                                break;
                            }
                            case "circle-segment": {
                                setDrawType(DrawActionType.CircleSegment);
                                drawManager.setActiveAction(DrawActionType.CircleSegment);
                                break;
                            }
                            case "arc-segment": {
                                setDrawType(DrawActionType.ArcSegment);
                                drawManager.setActiveAction(DrawActionType.ArcSegment);
                                break;
                            }
                        }
                    }}
                >
                    <Radio value="line-segment" label="Line Segment" checked={drawType == DrawActionType.LineSegment} />
                    <Radio
                        value="circle-segment"
                        label="Circle Segment"
                        checked={drawType == DrawActionType.CircleSegment}
                    />
                    <Radio
                        disabled
                        value="arc-segment"
                        label="Arc Segment"
                        checked={drawType == DrawActionType.ArcSegment}
                    />
                </RadioGroup>
                <Button
                    disabled={drawType == DrawActionType.None}
                    onClick={() => {
                        setDrawType(DrawActionType.None);
                        drawManager.setActiveAction(DrawActionType.None);
                    }}
                >
                    Stop
                </Button>
                <Button
                    disabled={drawType != DrawActionType.None}
                    onClick={() => {
                        useStageStore.getState().clear();
                    }}
                >
                    Clear
                </Button>
                {cost != null ? <Label>Cost: {cost}(ms)</Label> : null}
            </div>
            <div style={{ flex: 1 }} ref={stageContainerRef}>
                <Stage
                    onPointerDown={handlePointerDown}
                    onPointerMove={handlePointerMove}
                    onPointerUp={handlePointerUp}
                    width={width}
                    height={height}
                >
                    <Layer>
                        {lines.map((points, index) => (
                            <Line
                                key={"line-" + index}
                                points={[points[0].x, points[0].y, points[1].x, points[1].y]}
                                stroke="#df4b26"
                                strokeWidth={3}
                                tension={0.5}
                                lineCap="round"
                                lineJoin="round"
                            />
                        ))}
                        {tempLine ? (
                            <Line
                                key="line-temp"
                                points={[tempLine![0].x, tempLine![0].y, tempLine![1].x, tempLine![1].y]}
                                stroke="#df4b26"
                                strokeWidth={3}
                                tension={0.5}
                                lineCap="round"
                                lineJoin="round"
                            />
                        ) : null}
                        {circles.map((circle, index) => (
                            <Circle
                                key={"circle-" + index}
                                x={circle[0].x}
                                y={circle[0].y}
                                radius={circle[1]}
                                stroke="#df4b26"
                                strokeWidth={3}
                            />
                        ))}
                        {tempCircle ? (
                            <Circle
                                key="circle-temp"
                                x={tempCircle[0].x}
                                y={tempCircle[0].y}
                                radius={tempCircle[1]}
                                stroke="#df4b26"
                                strokeWidth={3}
                            />
                        ) : null}
                    </Layer>
                    <Layer>
                        {points.map((coords, index) => (
                            <Circle key={"point-" + index} x={coords[0].x} y={coords[0].y} radius={5} fill="black" />
                        ))}
                    </Layer>
                </Stage>
            </div>
        </div>
    );
}

export default App;
