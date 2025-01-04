"use client";
import {main} from "@/lib/test"
import {useEffect} from "react";
export function TestClient() {
    useEffect(() => {
        main()
    }, [])
    return <div>Hello</div>
}