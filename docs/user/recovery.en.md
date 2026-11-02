# If the PC behaves unexpectedly

Ametista is alpha software. Test in a disposable VM with a checkpoint, never on your main PC. Stop applying changes and keep the journal intact.

1. If Windows starts, open Ametista → History → Undo all. Review and confirm. Keep the original user account and investigate any reported conflict or failure. Interrupted temporary sessions must be recovered before starting another session
2. Removed apps are separate: Undo does not reinstall them. Use Open Store in removed-app history when available. Reinstallation may require internet and does not recover app data, the original version or provisioning
3. If necessary, use Control Panel → Recovery → Open System Restore and select a point from before the test. Review affected programs first. This restores system state and is not a personal-file backup. [Microsoft instructions](https://support.microsoft.com/en-us/windows/experience/backup-recovery/system-restore)
4. If Windows cannot start, use Windows Recovery Environment → Troubleshoot → Advanced options → System Restore. An encrypted device may require its BitLocker recovery key. Keep that key private. For a VM, restore the pre-test checkpoint instead. [Windows RE](https://support.microsoft.com/en-us/windows/experience/backup-recovery/windows-recovery-environment)

Do not delete/edit the journal, disable Windows security or reset/format the machine as a first troubleshooting step. Restore points must already exist; the app does not silently enable System Protection. [System Protection](https://support.microsoft.com/en-us/windows/experience/backup-recovery/system-protection).

For support, provide app version, Windows edition/build and the failed item/action. Review and anonymize diagnostics before sharing. Do not attach raw journals, personal paths, usernames, credentials or recovery keys. Report vulnerabilities through [SECURITY.md](../../SECURITY.md).
