# Nashira Deer // Hydrogen I18n

Translation utilities for server-side applications that need to deal with different languages at the same time.

[![PayPal](https://img.shields.io/badge/Paypal-003087?style=for-the-badge&logo=paypal&logoColor=%23fff)
](https://www.paypal.com/donate/?business=QQGMTC3FQAJF6&no_recurring=0&item_name=Thanks+for+donating+for+me%2C+this+helps+me+a+lot+to+continue+developing+and+maintaining+my+projects.&currency_code=USD)
[![GitHub Sponsor](https://img.shields.io/badge/GitHub%20Sponsor-181717?style=for-the-badge&logo=github&logoColor=%23fff)
](https://github.com/sponsors/nashiradeer)
[![Crates.io](https://img.shields.io/crates/v/hydrogen-i18n?style=for-the-badge&logo=rust&logoColor=%23fff&label=Crates.io&labelColor=%23000&color=%23000)
](https://crates.io/crates/hydrogen-i18n)
[![docs.rs](https://img.shields.io/docsrs/hydrogen-i18n?style=for-the-badge&logo=docsdotrs&logoColor=%23fff&label=Docs.rs&labelColor=%23000&color=%23000)
](https://docs.rs/hydrogen-i18n/)

Server-side applications that deal directly with users will need to be prepared to deal with different languages too, so Hydrogen I18n comes with utilities focused on making this task easier, loading and managing all the languages supported by the application in the memory, avoiding unnecessary disk and CPU usage.

Hydrogen I18n is part of [Hydrogen Framework](https://github.com/users/nashiradeer/projects/8), this means that this crate is designed to be used on Discord bots created using [serenity](https://crates.io/crates/serenity) and [twilight](https://crates.io/crates/twilight), but is sufficiently generic to be used by any other application that does not use these Discord libraries or even Discord bots.

## Donating

Consider donating to make Hydrogen I18n development possible. You can donate through Nashira Deer's [PayPal](https://www.paypal.com/donate/?business=QQGMTC3FQAJF6&no_recurring=0&item_name=Thanks+for+donating+for+me%2C+this+helps+me+a+lot+to+continue+developing+and+maintaining+my+projects.&currency_code=USD) or [GitHub Sponsor](https://github.com/sponsors/nashiradeer).

## Features

- `serenity`: Enables functions that make it easy to use the library in Discord apps and bots built with [serenity](https://crates.io/crates/serenity).
- `tokio`: Enables [tokio](https://crates.io/crates/tokio)-based builder.
- `simd`: Enables [simd-json](https://crates.io/crates/simd-json)-based parser and use it by default.

## Credits

Hydrogen I18n is a Nashira Deer project licensed under the [MIT License](https://github.com/nashiradeer/hydrogen-i18n/blob/main/LICENSE.txt) and licensed under the [GNU Lesser General Public License v3](https://github.com/nashiradeer/hydrogen-i18n/blob/c00b016356dc9263571e6cc6ede87969bf31bf02/LICENSE.txt) until v1.0.1.
