# T1 SDK CLI 

## Поддерживаемые операции

| Entity | List | Show | Delete | Create | CLI Command |
|--------|------|------|--------|-------------|--------|
| **Compute** | | | | |
| Server (Instance) | ✅ | - | - | - | `tilt-cli server list` |
| Flavor | - | - | - | - | `tilt-cli flavor list` |
| Image | ✅ | ✅ | - | - | `tilt-cli image list` / `tilt-cli image show <ID>` |
| SSH Key | ✅ | - | - | - | `tilt-cli ssh-key list` |
| Placement Policy | ✅ | - | - | - | `tilt-cli placement list` |
| **Storage** | | | | |
| Volume | ✅ | ✅ | - | - | `tilt-cli volume list` / `tilt-cli volume show <ID>` |
| Snapshot | ✅ | - | - | - | `tilt-cli snapshot list` |
| Backup | ✅ | - | - | - | `tilt-cli backup list` |
| **Network** | | | | |
| Network | ✅ | - | ✅ | - | `tilt-cli network list` / `tilt-cli network delete <ID>` |
| Subnet | ✅ | - | ✅ | - | `tilt-cli subnet list` / `tilt-cli subnet delete <ID>` |
| Port | ✅ | - | - | - | `tilt-cli port list` |
| Security Group | ✅ | - | ✅ | - | `tilt-cli security-group list` / `tilt-cli security-group show <ID>` / `tilt-cli security-group delete <ID>` |
| SNAT Router | ✅ | - | ✅ | - | `tilt-cli router list --type snat` / `tilt-cli router delete --type snat <ID>` |
| Network Router | ✅ | - | ✅ | - | `tilt-cli router list --type network` / `tilt-cli router delete --type network <ID>` |
| Route Table | ✅ | - | ✅ | - | `tilt-cli route-table list` / `tilt-cli route-table delete <ID>` |
| Virtual IP (VIP) | ✅ | - | ✅ | - | `tilt-cli vip list` / `tilt-cli vip delete <ID>` |
| Floating IP (FIP) | ✅ | - | ✅ | - | `tilt-cli fip list` / `tilt-cli fip delete <ID>` |
| **Topology** | | | | |
| Region | ✅ | - | - | - | `tilt-cli region list` |
| Availability Zone | ✅ | - | - | - | `tilt-cli az list` |

## Пагинация

Операция `list` поддерживает пагинацию
- `--limit <N>` - ограничение вывода сущностей (до 100)
- `-P` / `--page <N>` - номер страницы

**Примечание для роутеров:**
- Для SNAT роутеров (`--type snat`) пагинация не поддерживается
- Для network роутеров (`--type network`) пагинация поддерживается

## Дополнительные опции

- `--type <snat|network>` - тип роутера (только для команды `router`)
- `--long` - детальная информация
- `--format <table|json>` - формат вывода
