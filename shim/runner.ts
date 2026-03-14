import * as core from '@docusaurus/core';

const [,, command, siteDir, configPath] = process.argv;

if (!command || !siteDir || !configPath) {
  process.stderr.write('Usage: runner.js <command> <siteDir> <configPath>\n');
  process.exit(1);
}

type CoreKey = keyof typeof core;

void (async () => {
  const fn = core[command as CoreKey];
  if (typeof fn !== 'function') {
    process.stderr.write(`@docusaurus/core does not export '${command}' as a function\n`);
    process.exit(1);
  }
  await (fn as (siteDir: string, opts: { config: string }) => Promise<void>)(
    siteDir,
    { config: configPath },
  );
})();
