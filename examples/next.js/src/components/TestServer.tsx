"use server";
import {main} from "@/lib/test"
export async function TestServer() {
    await main()
    return <div>Hello</div>
}