"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const module_1 = require("module");
const [, , command, siteDir, configPath] = process.argv;
if (!command || !siteDir || !configPath) {
    process.stderr.write('Usage: runner.js <command> <siteDir> <configPath>\n');
    process.exit(1);
}
const req = (0, module_1.createRequire)(siteDir + '/');
const core = req('@docusaurus/core');
void (async () => {
    const fn = core[command];
    if (typeof fn !== 'function') {
        process.stderr.write(`@docusaurus/core does not export '${command}' as a function\n`);
        process.exit(1);
    }
    await fn(siteDir, { config: configPath });
})();
