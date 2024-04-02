import rollup_nre from "@rollup/plugin-node-resolve";
import rollup_tsc from "rollup-plugin-typescript2";

export default [
    {
        input: "src/main.ts",
        output: {
            file: "dist/js/app-bundle.js",
            format: "iife",
            name: "bundle",
            sourcemap: true
        },
        watch: true,
        plugins: [
            rollup_nre(),
            rollup_tsc({
                verbosity: 1,
                tsconfig: "tsconfig.json",
                tsconfigOverride: {
                    compilerOptions: {
                        declaration: false,
                        declarationMap: false,
                        sourceMap: true
                    }
                },
                useTsconfigDeclarationDir: false
            })
        ]
    }
]