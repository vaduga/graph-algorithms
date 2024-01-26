import { AnyValue } from "@/utils/types";
import GraphUniverse from "../GraphUniverse";

export type ComandConfiguraiton  = {
    delay: number;
}

export const DefaultCommandConfiguration : Readonly<ComandConfiguraiton> = {
    delay: 0
} as const;


export interface AlgorithmCommand<V = AnyValue, E = AnyValue> {
    execute(universe: GraphUniverse) : void;
    rollBack() : void;
    cofiguration(): ComandConfiguraiton;
}
