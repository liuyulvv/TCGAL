import { DrawAction, DrawActionType } from "./DrawAction";
import { DrawLineSegmentAction } from "./DrawLineSegmentAction";
import { DrawCircleSegmentAction } from "./DrawwCircleSegmentAction";

export class DrawManager {
    private static instance: DrawManager;
    private actions: Map<DrawActionType, DrawAction>;
    private activeAction: DrawAction | null;

    private constructor() {
        this.actions = new Map();
        this.activeAction = null;
        this.addAction(DrawLineSegmentAction.getInstance());
        this.addAction(DrawCircleSegmentAction.getInstance());
    }

    public static getInstance(): DrawManager {
        if (!DrawManager.instance) {
            DrawManager.instance = new DrawManager();
        }
        return DrawManager.instance;
    }

    public addAction(action: DrawAction): void {
        this.actions.set(action.actionType(), action);
    }

    public getAction(actionType: DrawActionType): DrawAction | undefined {
        return this.actions.get(actionType);
    }

    public removeAction(actionType: DrawActionType): void {
        this.actions.delete(actionType);
    }

    public setActiveAction(actionType: DrawActionType): void {
        this.activeAction?.onStop();
        this.activeAction = this.actions.get(actionType) || null;
    }

    public getActiveAction(): DrawAction | null {
        return this.activeAction;
    }
}
