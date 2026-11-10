//! The error messages of this crate, by catalogue key and English template
//! (ADR 0011). `bun run bindings` writes them to the frontend's catalogue.

homeport_core::messages! {
    // Pulls and pushes the registry refused

    /// A pull refused while Homeport had no login for the registry.
    PULL_REFUSED = "errors.registries.pullRefused" => "The image doesn't exist, or it's private.";
    PULL_REFUSED_HINT = "errors.registries.pullRefused.hint" => "Check the image name. If it's private, add a login for {registry} in Settings → Registries.";
    /// A pull refused although Homeport sent a stored login.
    PULL_REFUSED_LOGIN = "errors.registries.pullRefusedLogin" => "The image doesn't exist, or {username} can't access it.";
    PULL_REFUSED_LOGIN_HINT = "errors.registries.pullRefusedLogin.hint" => "Check the image name, and the login for {registry} in Settings → Registries.";
    /// The registry turned down the stored login itself.
    LOGIN_REFUSED = "errors.registries.loginRefused" => "{registry} refused the stored login of {username}.";
    LOGIN_REFUSED_HINT = "errors.registries.loginRefused.hint" => "Check the password or token for {registry} in Settings → Registries.";
    /// A push refused while Homeport had no login for the registry.
    PUSH_REFUSED = "errors.registries.pushRefused" => "{registry} refused the push.";
    PUSH_REFUSED_HINT = "errors.registries.pushRefused.hint" => "Add a login for {registry} in Settings → Registries.";
    /// A push refused although Homeport sent a stored login.
    PUSH_REFUSED_LOGIN = "errors.registries.pushRefusedLogin" => "{registry} refused the push from {username}.";
    PUSH_REFUSED_LOGIN_HINT = "errors.registries.pushRefusedLogin.hint" => "Check that this login may push to the repository, in Settings → Registries.";

    // Talking to registries

    /// The HTTPS client couldn't be built.
    HTTPS_CLIENT = "errors.registries.httpsClient" => "Homeport couldn't prepare its HTTPS client.";
    /// A request that never got an answer.
    UNREACHABLE = "errors.registries.unreachable" => "Homeport can't reach {address}.";
    UNREACHABLE_HINT = "errors.registries.unreachable.hint" => "Check the address, and this computer's internet access, DNS and proxy settings.";
    /// The registry answered 429.
    RATE_LIMITED = "errors.registries.rateLimited" => "{address} is limiting requests from this computer.";
    RATE_LIMITED_HINT = "errors.registries.rateLimited.hint" => "Wait a few minutes, then try again.";
    /// An answer that doesn't look like a registry's.
    UNEXPECTED = "errors.registries.unexpected" => "{address} gave an answer Homeport didn't expect.";
    UNEXPECTED_HINT = "errors.registries.unexpected.hint" => "Check that the address is a container registry. Its exact answer is in Details.";
    /// Checking a login the registry turned down.
    LOGIN_FAILED = "errors.registries.loginFailed" => "{address} refused this user name and password.";
    LOGIN_FAILED_HINT = "errors.registries.loginFailed.hint" => "Check them. Registries such as GHCR, and Docker Hub accounts with two-factor authentication, need an access token instead of the password.";

    // Docker Hub tags

    /// Listing tags of an image that isn't on Docker Hub.
    TAGS_ONLY_ON_HUB = "errors.registries.tagsOnlyOnHub" => "Tags can only be listed for Docker Hub repositories.";
    /// Listing tags of a Docker Hub repository that doesn't exist.
    HUB_NO_REPOSITORY = "errors.registries.hubNoRepository" => "Docker Hub has no repository named {repository}.";

    // Stored logins

    /// Saving a new login without its secret.
    SECRET_EMPTY = "errors.registries.secretEmpty" => "Enter the password or access token.";
    /// Saving a login for a registry that already has one.
    LOGIN_EXISTS = "errors.registries.loginExists" => "A login for {address} is already stored.";
    LOGIN_EXISTS_HINT = "errors.registries.loginExists.hint" => "Edit that login instead.";
    /// Saving a login without a usable registry address.
    ADDRESS_INVALID = "errors.registries.addressInvalid" => "Enter the registry's address, like ghcr.io or registry.example.com:5000.";
    /// Saving a login without a user name.
    USERNAME_EMPTY = "errors.registries.usernameEmpty" => "Enter the user name.";
    /// A login removed while it was in use.
    GONE = "errors.registries.gone" => "This registry login no longer exists.";
}
