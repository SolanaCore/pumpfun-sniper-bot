import { AnchorIdl, rootNodeFromAnchorWithoutDefaultVisitor } from "@codama/nodes-from-anchor";
import { renderJavaScriptUmiVisitor, renderJavaScriptVisitor, renderRustVisitor } from "@codama/renderers";
import { visit } from "@codama/visitors-core";
import anchorIdl from "./idl/pump_idl.json"; 

async function generateClients() {
    const node = rootNodeFromAnchorWithoutDefaultVisitor(anchorIdl as AnchorIdl);

    const clients = [
        { type: "Rust", dir: "rust/src", renderVisitor: renderRustVisitor }
    ];

    for (const client of clients) {
        try {
            await visit(
                node,
                await client.renderVisitor(client.dir)
            ); 
            console.log(`✅ Successfully generated ${client.type} client for directory: ${client.dir}!`);
        } catch (e) {
            console.error(`Error in ${client.renderVisitor.name}:`, e);
            throw e;
        }
    }
}

generateClients();