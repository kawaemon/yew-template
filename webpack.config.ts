import { ConfigurationFactory, CliConfigOptions } from "webpack";
import { resolve } from "path";
import * as CopyWebpackPlugin from "copy-webpack-plugin";

const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const DistPath = resolve(__dirname, "dist");

const WebpackConfig: ConfigurationFactory = (
  env: string,
  argv: CliConfigOptions
) => {
  return {
    entry: "./bootstrap.js",
    output: {
      path: DistPath,
      filename: "output.js",
    },
    plugins: [
      new CopyWebpackPlugin([{ from: "./static", to: DistPath }]),
      new WasmPackPlugin({
        crateDirectory: ".",
      }),
    ],

    devServer: {
      contentBase: DistPath,
      compress: argv.mode === "production",
      port: 8080,
    },
  };
};

export default WebpackConfig;
