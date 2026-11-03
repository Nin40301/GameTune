# What each adjustment does

This page is generated from the Ametista catalog. Simulate first and check the real state of your computer. A listed item may not be installed or supported by your Windows edition.

Appx is always excluded from automatic profiles. High-risk and unverified items are never preselected. The protection list may block an item even when documented here.

86 catalog items. Fonte: [items.json](../../catalog/items.json).

## Disable classic Copilot

`reg.copilot` · `registry`

- **What changes:** Disable classic Copilot by policy. The standalone Copilot app may be unaffected. Undo restores original type and bytes or removes the created value
- **Risk:** Low
- **What may stop working:** The classic Copilot panel may become unavailable
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-windowsai#turnoffwindowscopilot)

## Hide Copilot button

`reg.copilot_button` · `registry`

- **What changes:** Hide the Copilot taskbar button. Undo restores original type and bytes or removes the created value
- **Risk:** Low
- **What may stop working:** The visual Copilot shortcut is hidden
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Unverified — excluded from automatic profiles
- **Declared editions:** pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: Unverified — excluded from automatic profiles

## Disable Recall analysis

`reg.recall` · `registry`

- **What changes:** Request disabling Recall AI analysis. Undo restores original type and bytes or removes the created value
- **Risk:** High — manual selection and reinforced consent
- **What may stop working:** May delete existing Recall history. Restoring the registry does not recover that history. Action blocked
- **How to restore:** No guaranteed automatic restoration. Check risks and sources; protection may refuse this item
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** pro, enterprise, education
- **Minimum build:** 26100
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-windowsai#disableaidataanalysis)

## Remove Microsoft.Copilot

`appx.microsoft.copilot` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Remove Microsoft.Windows.Ai.Copilot.Provider

`appx.microsoft.windows.ai.copilot.provider` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Safe
- **Verification:** Unverified — excluded from automatic profiles
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: Unverified — excluded from automatic profiles

## Remove Microsoft.549981C3F5F10

`appx.microsoft.549981c3f5f10` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Disable Cortana

`reg.cortana` · `registry`

- **What changes:** Disable Cortana by policy on supported editions. Undo restores original type and bytes or removes the created value
- **Risk:** Low
- **What may stop working:** The Cortana assistant becomes unavailable. Does not change file search
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-experience#allowcortana)

## Remove Microsoft.GamingApp

`appx.microsoft.gamingapp` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Remove Microsoft.XboxApp

`appx.microsoft.xboxapp` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remove Microsoft.XboxGamingOverlay

`appx.microsoft.xboxgamingoverlay` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Remove Microsoft.XboxGameOverlay

`appx.microsoft.xboxgameoverlay` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remove Microsoft.XboxSpeechToTextOverlay

`appx.microsoft.xboxspeechtotextoverlay` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remove Microsoft.XboxIdentityProvider

`appx.microsoft.xboxidentityprovider` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** High — manual selection and reinforced consent
- **What may stop working:** May prevent Minecraft Bedrock, Game Pass and Microsoft Store games from launching or signing in
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Aggressive
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remove Microsoft.GamingServices

`appx.microsoft.gamingservices` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** High — manual selection and reinforced consent
- **What may stop working:** May prevent Minecraft Bedrock, Game Pass and Microsoft Store games from launching or signing in
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Aggressive
- **Verification:** Unverified — excluded from automatic profiles
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: Unverified — excluded from automatic profiles

## Disable Xbox authentication

`svc.xblauthmanager` · `service`

- **What changes:** Stop Xbox Live authentication. Undo restores startup mode and running state
- **Risk:** High — manual selection and reinforced consent
- **What may stop working:** May prevent Minecraft Bedrock, Game Pass and Microsoft Store games from launching or signing in
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Aggressive
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Disable Xbox game saves

`svc.xblgamesave` · `service`

- **What changes:** Stop game save synchronization. Undo restores startup mode and running state
- **Risk:** High — manual selection and reinforced consent
- **What may stop working:** May prevent Minecraft Bedrock, Game Pass and Microsoft Store games from launching or signing in
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Aggressive
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Disable Xbox Live networking

`svc.xboxnetapisvc` · `service`

- **What changes:** Provide Xbox Live game connectivity. Undo restores startup mode and running state
- **Risk:** High — manual selection and reinforced consent
- **What may stop working:** May prevent Minecraft Bedrock, Game Pass and Microsoft Store games from launching or signing in — Blocked by network service protection
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Aggressive
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Disable Xbox accessories

`svc.xboxgipsvc` · `service`

- **What changes:** Stop Xbox accessory communication. Undo restores startup mode and running state
- **Risk:** High — manual selection and reinforced consent
- **What may stop working:** May stop wireless Xbox controllers or their adapters from working
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Aggressive
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Remove Microsoft.BingNews

`appx.microsoft.bingnews` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Remove Microsoft.BingWeather

`appx.microsoft.bingweather` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remove Microsoft.GetHelp

`appx.microsoft.gethelp` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remove Microsoft.Getstarted

`appx.microsoft.getstarted` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remove Microsoft.WindowsFeedbackHub

`appx.microsoft.windowsfeedbackhub` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remove Microsoft.MicrosoftOfficeHub

`appx.microsoft.microsoftofficehub` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remove Microsoft.People

`appx.microsoft.people` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remove Microsoft.MixedReality.Portal

`appx.microsoft.mixedreality.portal` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Remove Microsoft.Microsoft3DViewer

`appx.microsoft.microsoft3dviewer` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remove Microsoft.Print3D

`appx.microsoft.print3d` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remove Microsoft.SkypeApp

`appx.microsoft.skypeapp` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remove MicrosoftTeams

`appx.microsoftteams` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Remove MSTeams

`appx.msteams` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Remove Clipchamp.Clipchamp

`appx.clipchamp.clipchamp` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Remove Microsoft.Todos

`appx.microsoft.todos` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Remove Microsoft.PowerAutomateDesktop

`appx.microsoft.powerautomatedesktop` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Remove Microsoft.YourPhone

`appx.microsoft.yourphone` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Remove Microsoft.WindowsMaps

`appx.microsoft.windowsmaps` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remove MicrosoftCorporationII.MicrosoftFamily

`appx.microsoftcorporationii.microsoftfamily` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Balanced
- **Verification:** Unverified — excluded from automatic profiles
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: Unverified — excluded from automatic profiles

## Remove Microsoft.Windows.DevHome

`appx.microsoft.windows.devhome` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Remove Microsoft.ZuneVideo

`appx.microsoft.zunevideo` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remove microsoft.windowscommunicationsapps

`appx.microsoft.windowscommunicationsapps` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remove Microsoft.MicrosoftSolitaireCollection

`appx.microsoft.microsoftsolitairecollection` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remove Microsoft.ZuneMusic

`appx.microsoft.zunemusic` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Aggressive
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remove Microsoft.OutlookForWindows

`appx.microsoft.outlookforwindows` · `appx`

- **What changes:** Remove the installed app and its provisioning. Manual reinstall may need internet and cannot recover data or the original version
- **Risk:** Low
- **What may stop working:** The app becomes unavailable. Manual reinstall cannot recover data or guarantee the original version
- **How to restore:** Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store
- **Level:** Aggressive
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Disable Connected diagnostics

`svc.diagtrack` · `service`

- **What changes:** Reduce background diagnostic collection. Undo restores startup mode and running state
- **Risk:** Low
- **What may stop working:** May reduce data available to troubleshoot failures
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Disable Device management messages

`svc.dmwappushservice` · `service`

- **What changes:** Stop routing device management messages. Undo restores startup mode and running state
- **Risk:** Low
- **What may stop working:** May affect devices managed by an organization
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Disable Diagnostic collector

`svc.diagnosticshub.standardcollector.service` · `service`

- **What changes:** Stop developer diagnostic collection. Undo restores startup mode and running state
- **Risk:** Low
- **What may stop working:** Diagnostic tools may stop collecting data
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Disable Retail demonstration

`svc.retaildemo` · `service`

- **What changes:** Disable retail demonstration mode. Undo restores startup mode and running state
- **Risk:** Low
- **What may stop working:** Retail demonstration mode stops working
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Disable Media sharing

`svc.wmpnetworksvc` · `service`

- **What changes:** Share media over the network. Undo restores startup mode and running state
- **Risk:** Low
- **What may stop working:** Blocked by network service protection
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Disable Remote registry

`svc.remoteregistry` · `service`

- **What changes:** Disable remote registry access. Undo restores startup mode and running state
- **Risk:** Low
- **What may stop working:** May affect organization remote support
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Disable Fax

`svc.fax` · `service`

- **What changes:** Disable sending and receiving faxes. Undo restores startup mode and running state
- **Risk:** Low
- **What may stop working:** Fax stops working
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Disable Error reporting

`svc.wersvc` · `service`

- **What changes:** Stop background error reporting. Undo restores startup mode and running state
- **Risk:** Low
- **What may stop working:** May reduce information available to investigate crashes
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Disable Downloaded maps

`svc.mapsbroker` · `service`

- **What changes:** Stop offline map management. Undo restores startup mode and running state
- **Risk:** Low
- **What may stop working:** Offline maps may stop updating
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Disable Windows Insider program

`svc.wisvc` · `service`

- **What changes:** Disable Windows preview service. Undo restores startup mode and running state
- **Risk:** Low
- **What may stop working:** May affect Windows Insider enrollment
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Disable Phone integration

`svc.phonesvc` · `service`

- **What changes:** Disable phone integration. Undo restores startup mode and running state
- **Risk:** Low
- **What may stop working:** Integrated phone features may stop
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Disable Payments and NFC

`svc.semgrsvc` · `service`

- **What changes:** Disable payments and NFC management. Undo restores startup mode and running state
- **Risk:** Low
- **What may stop working:** Payments and NFC may stop working
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Disable Mobile hotspot

`svc.icssvc` · `service`

- **What changes:** Share internet with other devices. Undo restores startup mode and running state
- **Risk:** Low
- **What may stop working:** Blocked by network service protection
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Use manual startup for Delivery Optimization

`svc.dosvc` · `service`

- **What changes:** Set Delivery Optimization startup to manual. Undo restores startup mode and running state
- **Risk:** Low
- **What may stop working:** May affect peer download distribution. Does not disable Windows Update
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Disable Location

`svc.lfsvc` · `service`

- **What changes:** Disable device location. Undo restores startup mode and running state
- **Risk:** Low
- **What may stop working:** Apps that use location may fail
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Aggressive
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Disable File indexing

`svc.wsearch` · `service`

- **What changes:** Stop file indexing. Undo restores startup mode and running state
- **Risk:** Low
- **What may stop working:** File search may become slower or incomplete
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Aggressive
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Disable App preloading

`svc.sysmain` · `service`

- **What changes:** Stop app preloading. Undo restores startup mode and running state
- **Risk:** Low
- **What may stop working:** Preselect only on SSD or NVMe. Loading may become slower
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Reduce diagnostic data

`reg.telemetry` · `registry`

- **What changes:** Request the configured minimum diagnostic level. Results depend on edition. Undo restores original type and bytes or removes the created value
- **Risk:** Low
- **What may stop working:** The edition may enforce another minimum level. Does not guarantee zero traffic
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-system#allowtelemetry)

## Disable Microsoft Compatibility Appraiser

`task.microsoft_compatibility_appraiser` · `scheduled_task`

- **What changes:** Disable this scheduled task. Undo restores its enabled state
- **Risk:** Low
- **What may stop working:** Diagnostic reports or map updates may stop
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Unverified — excluded from automatic profiles
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration)

## Disable ProgramDataUpdater

`task.programdataupdater` · `scheduled_task`

- **What changes:** Disable this scheduled task. Undo restores its enabled state
- **Risk:** Low
- **What may stop working:** Diagnostic reports or map updates may stop
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Unverified — excluded from automatic profiles
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration)

## Disable Consolidator

`task.consolidator` · `scheduled_task`

- **What changes:** Disable this scheduled task. Undo restores its enabled state
- **Risk:** Low
- **What may stop working:** Diagnostic reports or map updates may stop
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Unverified — excluded from automatic profiles
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration)

## Disable UsbCeip

`task.usbceip` · `scheduled_task`

- **What changes:** Disable this scheduled task. Undo restores its enabled state
- **Risk:** Low
- **What may stop working:** Diagnostic reports or map updates may stop
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Unverified — excluded from automatic profiles
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration)

## Disable DmClient

`task.dmclient` · `scheduled_task`

- **What changes:** Disable this scheduled task. Undo restores its enabled state
- **Risk:** Low
- **What may stop working:** Diagnostic reports or map updates may stop
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Unverified — excluded from automatic profiles
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration)

## Disable QueueReporting

`task.queuereporting` · `scheduled_task`

- **What changes:** Disable this scheduled task. Undo restores its enabled state
- **Risk:** Low
- **What may stop working:** Diagnostic reports or map updates may stop
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Unverified — excluded from automatic profiles
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration)

## Disable MapsToastTask

`task.mapstoasttask` · `scheduled_task`

- **What changes:** Disable this scheduled task. Undo restores its enabled state
- **Risk:** Low
- **What may stop working:** Diagnostic reports or map updates may stop
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Unverified — excluded from automatic profiles
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration)

## Disable MapsUpdateTask

`task.mapsupdatetask` · `scheduled_task`

- **What changes:** Disable this scheduled task. Undo restores its enabled state
- **Risk:** Low
- **What may stop working:** Diagnostic reports or map updates may stop
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Unverified — excluded from automatic profiles
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration)

## Disable app suggestions

`reg.consumer_features` · `registry`

- **What changes:** Disable Windows promotional experiences where the policy is supported. Undo restores original type and bytes or removes the created value
- **Risk:** Low
- **What may stop working:** The policy may be ignored on some editions
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-experience#allowwindowsconsumerfeatures)

## Disable advertising ID

`reg.advertising` · `registry`

- **What changes:** Disable the identifier used for personalized advertising. Undo restores original type and bytes or removes the created value
- **Risk:** Low
- **What may stop working:** Ads may no longer be personalized
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-privacy#disableadvertisingid)

## Disable Widgets

`reg.widgets` · `registry`

- **What changes:** Disable the Widgets panel where the policy is supported. Undo restores original type and bytes or removes the created value
- **Risk:** Low
- **What may stop working:** Panel news and weather become unavailable
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** pro, enterprise, education
- **Minimum build:** 22000
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-newsandinterests#allownewsandinterests)

## Disable suggestion SystemPaneSuggestionsEnabled

`reg.systempanesuggestionsenabled` · `registry`

- **What changes:** Disable automatic promotional suggestions or installations. Undo restores the original value
- **Risk:** Low
- **What may stop working:** Windows suggestions may no longer appear
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/DefaultUserSettings.json)

## Disable suggestion SilentInstalledAppsEnabled

`reg.silentinstalledappsenabled` · `registry`

- **What changes:** Disable automatic promotional suggestions or installations. Undo restores the original value
- **Risk:** Low
- **What may stop working:** Windows suggestions may no longer appear
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Unverified — excluded from automatic profiles
- **Declared editions:** pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: Unverified — excluded from automatic profiles

## Disable suggestion SubscribedContent-338389Enabled

`reg.subscribedcontent-338389enabled` · `registry`

- **What changes:** Disable automatic promotional suggestions or installations. Undo restores the original value
- **Risk:** Low
- **What may stop working:** Windows suggestions may no longer appear
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Unverified — excluded from automatic profiles
- **Declared editions:** pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: Unverified — excluded from automatic profiles

## Disable Start recommendations

`reg.iris` · `registry`

- **What changes:** Request disabling Start menu recommendations. Undo restores original type and bytes or removes the created value
- **Risk:** Low
- **What may stop working:** Recommendations may disappear from Start
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 22000
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/apps/develop/settings/settings-windows-11)

## Disable background game recording

`reg.game_dvr_policy` · `registry`

- **What changes:** Disable game recording and broadcasting by policy on Windows 10. Undo restores original type and bytes or removes the created value
- **Risk:** Low
- **What may stop working:** Windows game recording becomes unavailable
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-applicationmanagement#allowgamedvr)

## Disable user Game DVR

`reg.game_dvr_user` · `registry`

- **What changes:** Disable Game DVR for the interactive user. Undo restores original type and bytes or removes the created value
- **Risk:** Low
- **What may stop working:** Background game recording may stop
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Unverified — excluded from automatic profiles
- **Declared editions:** pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: Unverified — excluded from automatic profiles

## Enable Game Mode

`reg.game_mode` · `registry`

- **What changes:** Enable the Windows Game Mode preference. Undo restores original type and bytes or removes the created value
- **Risk:** Low
- **What may stop working:** Results vary by game and must be measured
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Safe
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 22000
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/apps/develop/settings/settings-windows-11)

## Disable Edge startup boost

`reg.edge_preload` · `registry`

- **What changes:** Disable Edge startup preloading. Undo restores original type and bytes or removes the created value
- **Risk:** Low
- **What may stop working:** The first Edge launch may take longer
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/deployedge/microsoft-edge-policies/startupboostenabled)

## Disable Edge background mode

`reg.edge_background` · `registry`

- **What changes:** Disable Edge background execution. Undo restores original type and bytes or removes the created value
- **Risk:** Low
- **What may stop working:** Background notifications and extensions may stop
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/deployedge/microsoft-edge-policies/backgroundmodeenabled)

## Enable hardware GPU scheduling

`reg.hags` · `registry`

- **What changes:** Request hardware accelerated GPU scheduling. Undo restores original type and bytes or removes the created value
- **Risk:** Low
- **What may stop working:** Requires compatible GPU and driver and restart. Performance may decrease
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Balanced
- **Verification:** Unverified — excluded from automatic profiles
- **Declared editions:** pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** Yes

Sources: Unverified — excluded from automatic profiles

## Enable High Performance

`power.high_performance` · `power`

- **What changes:** Activate High Performance. Heat and power use may increase. Undo restores the previous plan and removes only plans created by the app
- **Risk:** Low
- **What may stop working:** May affect availability of this feature
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Balanced
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/win32/power/power-setting-guids), [2](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options)

## Block background apps

`reg.background_apps` · `registry`

- **What changes:** Block Store app background execution. Undo restores original type and bytes or removes the created value
- **Risk:** High — manual selection and reinforced consent
- **What may stop working:** Store apps such as WhatsApp may stop notifying when closed
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Aggressive
- **Verification:** Official source recorded; compatibility still depends on Windows version/edition
- **Declared editions:** pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: [1](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-privacy#letappsruninbackground)

## Review startup HKCU

`startup.hkcu` · `startup`

- **What changes:** Discover startup entries. Select each entry manually. Essential entries stay hidden
- **Risk:** Low
- **What may stop working:** May affect availability of this feature
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Manual only
- **Verification:** Unverified — excluded from automatic profiles
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: Unverified — excluded from automatic profiles

## Review startup HKLM

`startup.hklm` · `startup`

- **What changes:** Discover startup entries. Select each entry manually. Essential entries stay hidden
- **Risk:** Low
- **What may stop working:** May affect availability of this feature
- **How to restore:** Use Undo to restore the original state recorded before applying
- **Level:** Manual only
- **Verification:** Unverified — excluded from automatic profiles
- **Declared editions:** home, pro, enterprise, education
- **Minimum build:** 19045
- **Restart:** No

Sources: Unverified — excluded from automatic profiles
