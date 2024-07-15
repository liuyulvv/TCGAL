import { Button, Radio, RadioGroup } from "@fluentui/react-components";
import { KonvaEventObject } from "konva/lib/Node";
import { useEffect, useRef, useState } from "react";
import { Layer, Line, Stage } from "react-konva";
import { DrawActionType } from "./draw/DrawAction";
import { DrawManager } from "./draw/DrawManager";
import { useStageStore } from "./status/Stage";

function App() {
    const stageContainerRef = useRef<HTMLDivElement>(null);
    const lines = useStageStore((state) => state.lines);
    const tempLine = useStageStore((state) => state.tempLine);
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
                        disabled
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
                                key={index}
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
                                key={-1}
                                points={[tempLine![0].x, tempLine![0].y, tempLine![1].x, tempLine![1].y]}
                                stroke="#df4b26"
                                strokeWidth={3}
                                tension={0.5}
                                lineCap="round"
                                lineJoin="round"
                            />
                        ) : null}
                    </Layer>
                </Stage>
            </div>
        </div>
    );
}

export default App;
