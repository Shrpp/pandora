# Admin TUI

The `ovlt` binary is a terminal UI for managing OVLT. It connects to a running OVLT server over HTTP.

---

## Install

Download from [GitHub Releases](https://github.com/shrpp/ovlt/releases/latest).

**macOS**
```bash
# M1/M2/M3
curl -Lo ovlt https://github.com/shrpp/ovlt/releases/latest/download/ovlt-aarch64-apple-darwin
# Intel
curl -Lo ovlt https://github.com/shrpp/ovlt/releases/latest/download/ovlt-x86_64-apple-darwin

xattr -dr com.apple.quarantine ovlt   # required — binary is unsigned in alpha
chmod +x ovlt && sudo mv ovlt /usr/local/bin/
```

**Linux**
```bash
# x86_64
curl -Lo ovlt https://github.com/shrpp/ovlt/releases/latest/download/ovlt-x86_64-unknown-linux-gnu
# ARM64
curl -Lo ovlt https://github.com/shrpp/ovlt/releases/latest/download/ovlt-aarch64-unknown-linux-gnu

chmod +x ovlt && sudo mv ovlt /usr/local/bin/
```

**Windows**
```powershell
curl -Lo ovlt.exe https://github.com/shrpp/ovlt/releases/latest/download/ovlt-x86_64-pc-windows-msvc.exe
.\ovlt.exe --url http://localhost:3000
```
> Windows SmartScreen will show a warning because the binary is not yet code-signed. This is expected for alpha builds. Click **More info → Run anyway** to proceed.

## Connect

```bash
ovlt --url http://localhost:3000
# or
OVLT_URL=http://localhost:3000 ovlt
```

On launch you are prompted for the **Admin Key** (`OVLT_ADMIN_KEY` from server config).

---

## Navigation

| Key | Action |
|-----|--------|
| `Tab` / `Shift+Tab` | Move between tabs |
| `↑` / `↓` or `j` / `k` | Move selection |
| `Enter` | Open / confirm |
| `Esc` | Close modal / cancel |
| `n` | New item |
| `d` | Delete selected |
| `e` | Edit selected |
| `r` | Roles (M2M clients only) |
| `?` | Toggle help |
| `q` | Quit |

---

## Tabs

### Tenants

List, create, and manage tenants. Each tenant is isolated — users, clients, and roles belong to a single tenant.

### Users

Lists all users in the selected tenant. You can:
- Create users
- View / edit user details
- Delete users
- Reset passwords (generates a reset token)
- Get verification codes

### Clients

OAuth 2.0 clients within the selected tenant.

Fields:
- **Name** — display name
- **Client ID** — auto-generated
- **Client Secret** — auto-generated; shown once
- **Grant Types** — `authorization_code`, `client_credentials`, or both
- **Redirect URIs** — required for `authorization_code`
- **Scopes** — space-separated allowed scopes

For `client_credentials` (M2M) clients, press `r` to assign roles.

### Roles

Roles for the selected tenant. Roles can be assigned to users or to M2M clients.

### Permissions

Fine-grained permissions. Permissions are assigned to roles, which are then assigned to users or clients.

### Sessions

Active sessions for the tenant. You can revoke a session by pressing `d`.

### Audit Log

Read-only view of all auth events (logins, logouts, failures, MFA events, token issues) for the tenant.

---

## Tips for agents

- The TUI requires a terminal — use the [API Reference](api-reference.md) for scripted automation
- All TUI operations map 1:1 to API endpoints under `/tenants`, `/users`, `/clients`, `/roles`, `/permissions`
- `OVLT_URL` env var eliminates the `--url` flag
