# [unreleased]

# 0.1.0

Breaking changes:

* Upgrade public dependencies

Improvements:

* Add more endpoints:
  ```rust
  association::{
      email::{
          create_email_validation_session::v2,
          validate_email::v2,
          validate_email_by_end_user::v2,
      },
      msisdn::{
          create_msisdn_validation_session::v2,
          validate_msisdn::v2,
          validate_msisdn_by_phone_number::v2,
      },
  },
  key::{
      check_public_key_validity::v2,
      get_public_key::v2,
      validate_ephemeral_key::v2,
  },
  lookup::{
      get_hash_parameters::v2,
      lookup_3pid::v2,
  },
  status::v2,
  tos::{
      accept_terms_of_service::v2,
      get_terms_of_service::v2,
  }
  ```

# 0.0.1

Initial release with the following endpoints:

```rust
authentication::{get_account_information::v2, logout::v2, register::v2}
```
