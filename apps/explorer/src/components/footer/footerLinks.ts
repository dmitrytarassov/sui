// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

type FooterItem = { title: string; href: string }[];

export type FooterItems = FooterItem[];
export const footerLinks = [
    { title: 'Blog', href: 'https://medium.com/mysten-labs' },
    {
        title: 'Whitepaper',
        href: 'https://github.com/MystenLabs/sui/blob/main/doc/paper/sui.pdf',
    },
    { title: 'Press', href: 'https://mystenlabs.com/#community' },
    {
        title: 'Docs',
        href: 'https://docs.sui.io/',
    },
    {
        title: 'GitHub',
        href: 'https://github.com/MystenLabs',
    },
    {
        title: 'Discord',
        href: 'https://discord.gg/sui',
    },
    {
        title: 'Twitter',
        href: 'https://twitter.com/SuiNetwork',
    },
    {
        title: 'LinkedIn',
        href: 'https://www.linkedin.com/company/mysten-labs/',
    },
    {
        title: 'Terms & Conditions',
        href: 'https://mystenlabs.com/legal?content=terms',
    },
    {
        title: 'Privacy Policy',
        href: 'https://mystenlabs.com/legal?content=privacy',
    },
];
