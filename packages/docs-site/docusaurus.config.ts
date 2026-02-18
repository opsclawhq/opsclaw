import type { Config } from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

const config: Config = {
  title: 'OpsClaw Docs',
  tagline: 'Build, run, and operate AI SRE squads',
  url: 'https://opsclawhq.github.io',
  baseUrl: '/',
  organizationName: 'opsclawhq',
  projectName: 'opsclaw',
  onBrokenLinks: 'warn',
  markdown: {
    hooks: {
      onBrokenMarkdownLinks: 'warn',
    },
  },
  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },
  presets: [
    [
      'classic',
      {
        docs: {
          path: '../../docs',
          routeBasePath: 'docs',
          sidebarPath: './sidebars.ts',
          exclude: ['plans/**', 'api/**/*.yaml'],
          editUrl: 'https://github.com/opsclawhq/opsclaw/tree/main/',
        },
        blog: false,
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],
  themeConfig: {
    navbar: {
      title: 'OpsClaw',
      items: [
        { type: 'doc', docId: 'getting-started', position: 'left', label: 'Getting Started' },
        { type: 'doc', docId: 'user-guide/README', position: 'left', label: 'User Guide' },
        { type: 'doc', docId: 'developer-guide/README', position: 'left', label: 'Developer Guide' },
        { to: '/docs/blog', position: 'left', label: 'Engineering Blog' },
        {
          href: 'https://github.com/opsclawhq/opsclaw',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'Docs',
          items: [
            { label: 'Getting Started', to: '/docs/getting-started' },
            { label: 'User Guide', to: '/docs/user-guide' },
            { label: 'Developer Guide', to: '/docs/developer-guide' },
          ],
        },
        {
          title: 'Community',
          items: [
            {
              label: 'GitHub',
              href: 'https://github.com/opsclawhq/opsclaw',
            },
          ],
        },
      ],
      copyright: `Copyright ${new Date().getFullYear()} OpsClaw`,
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
