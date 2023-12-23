import GraphUniverse from "./GraphUniverse";
import GraphUniverseComponent from "@/GraphUniverse/GraphUniverseComponent";

export default class GraphUniverseCamera<T> implements GraphUniverseComponent<T> {
    private universe: GraphUniverse<object>;

    constructor(universe: GraphUniverse<object>) {
        this.universe = universe;
    }

    public initialize(): void {
        this.initializeEventListener();
        this.initializeZoomController();
    }

    private initializeZoomController() {
        this.universe.viewport
            .pinch()
            .wheel()
            .decelerate();
    }

    private initializeEventListener(): void {
        window.addEventListener('keydown', (e) => {

            let delta_x = 0;
            let delta_y = 0;

            if (e.key == "ArrowUp") {
                delta_y = -10;
            }

            if (e.key == "ArrowDown") {
                delta_y = 10;
            }

            if (e.key == "ArrowLeft") {
                delta_x = -10;
            }

            if (e.key == "ArrowRight") {
                delta_x = +10;
            }

            this.universe.viewport.moveCenter(
                this.universe.viewport.center.x + delta_x,
                this.universe.viewport.center.y + delta_y
            )
        });
    }
}