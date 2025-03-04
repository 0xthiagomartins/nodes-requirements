import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Node } from "@/types/node"
import { fetchApi } from "@/lib/api"

async function getNodes() {
    return fetchApi<Node[]>('/nodes')
}

export async function NodesList() {
    const nodes = await getNodes()

    return (
        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
            {nodes.map((node: Node) => (
                <Card key={node.id}>
                    <CardHeader>
                        <CardTitle className="capitalize">{node.blockchain_type}</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <dl className="space-y-2 text-sm">
                            <div>
                                <dt className="text-gray-500">CPU Cores</dt>
                                <dd>{node.cpu_cores}</dd>
                            </div>
                            <div>
                                <dt className="text-gray-500">RAM</dt>
                                <dd>{node.ram_gb}GB</dd>
                            </div>
                            <div>
                                <dt className="text-gray-500">Storage</dt>
                                <dd>{node.storage_gb}GB</dd>
                            </div>
                            <div>
                                <dt className="text-gray-500">Network</dt>
                                <dd>{node.network_mbps}Mbps</dd>
                            </div>
                        </dl>
                    </CardContent>
                </Card>
            ))}
        </div>
    )
} 