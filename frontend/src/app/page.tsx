import { Suspense } from "react"
import { NodesList } from "@/components/nodes/NodesList"
import { NodesLoading } from "@/components/nodes/NodesLoading"

export default function Home() {
    return (
        <div className="space-y-8">
            <div>
                <h2 className="text-3xl font-bold tracking-tight">Blockchain Nodes</h2>
                <p className="text-muted-foreground">
                    Compare hardware requirements for different blockchain nodes
                </p>
            </div>
            <Suspense fallback={<NodesLoading />}>
                <NodesList />
            </Suspense>
        </div>
    )
}
