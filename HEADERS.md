# Headers from the Windows SDK

This page should list all of the headers in the default `Include/<version>/shared` folder for the Windows 10 SDK. For dependency resolution on other headers, the UCRT headers were taken as a starting point, but other SDK-specific headers might be feature includes later. The other directories in the versioned SDK might at some point be added, if they prove to contain more useful things not provided through the existing projections.

Entries for which the headers have not yet been tracked are subject to become unplanned if it turns out they don't fit the project. Driver and device interfaces will not be supported here unless someone else comes on board to maintain them. There's some stuff not (yet) worth using.

| Directory | File | Planned  | Implemented | Notes |
|-----------|------|----------|-------------|-------|
| shared/ndis | *  | &#x2718; | | Network Driver stuff, will do later |
| shared/netcx | * | &#x2718; | | Network Adapter Class Extensions, will do later |
| shared    | afunix.h | &#x2718; | | At least not until [this is fixed](https://github.com/microsoft/WSL/issues/4240). |
| shared    | apdevpkey.h | &#x2718; | | Nothing to do here |
| shared    | apiset.h | &#x2718; | | Nothing to do here |
| shared    | apisetcconv.h | &#x2718; | | Nothing to do here |
| shared    | atsmedia.h | &#x2718; | | Nothing to do here |
| shared    | basetsd.h | &#x2718; | | Nothing to do here |
| shared    | basetyps.h | &#x2718; | | Nothing to do here |
| shared    | batclass.h | &#x2718; | | Nothing to do here |
| shared    | bcrypt.h | &#x2718; | | Nothing to do here |
| shared    | bdamedia.h | &#x2718; | | Nothing to do here |
| shared    | bdatypes.h | &#x2718; | | Nothing to do here |
| shared    | bthdef.h | &#x2718; | | Nothing to do here |
| shared    | bthioctl.h | &#x2718; | | Nothing to do here |
| shared    | bthsdpdef.h | &#x2718; | | Nothing to do here |
| shared    | bugcodes.h | &#x2718; | | Nothing to do here |
| shared    | cderr.h | &#x2718; | | Nothing to do here |
| shared    | cfg.h | &#x2718; | | Nothing to do here |
| shared    | clfs.h | &#x2718; | | Nothing to do here |
| shared    | clfslsn.h | &#x2718; | | Nothing to do here |
| shared    | common.ver | &#x2718; | | Nothing to do here |
| shared    | concurrencysal.h | &#x2718; | | Nothing to do here |
| shared    | d3d9.h | &#x2718; | | Nothing to do here |
| shared    | d3d9caps.h | &#x2718; | | Nothing to do here |
| shared    | d3d9types.h | &#x2718; | | Nothing to do here |
| shared    | d3dkmdt.h | &#x2718; | | Nothing to do here |
| shared    | d3dkmthk.h | &#x2718; | | Nothing to do here |
| shared    | d3dukmdt.h | &#x2718; | | Nothing to do here |
| shared    | dciddi.h | &#x2718; | | Nothing to do here |
| shared    | dcomptypes.h | &#x2718; | | Nothing to do here |
| shared    | devguid.h | &#x2718; | | Nothing to do here |
| shared    | devioctl.h | &#x2718; | | Nothing to do here |
| shared    | devpkey.h | &#x2718; | | Nothing to do here |
| shared    | devpropdef.h | &#x2718; | | Nothing to do here |
| shared    | dinputd.h | &#x2718; | | Nothing to do here |
| shared    | diskguid.h | &#x2718; | | Nothing to do here |
| shared    | dls1.h | &#x2718; | | Nothing to do here |
| shared    | dls2.h | &#x2718; | | Nothing to do here |
| shared    | dmdls.h | &#x2714; | &#x2718; | Seems to only be the MakeFourCC macro which turns a string into a single byte `OR`ed value, might also be defined elsewhere? |
| shared    | dmerror.h | &#x2714; | &#x2718; | `MakeHResult` seems interesting, but might be defined elsewhere too. DMH Error and Success wrappers could be interesting too, if DirectMusic has projections. |
| shared    | dmusbuff.h | &#x2714; | &#x2718; | `DMUS_EVENT_SIZE` if DirectMusic has projections, QWORD_ALIGN seems cool in general. |
| shared    | dontuse.h | &#x2718; | | This just adds deprecation warnings to C(++) functions and is not relevant to us. |
| shared    | dpfilter.h | &#x2718; | | Just a typedef |
| shared    | driverspecs.h | &#x2718; | | Out of scope |
| shared    | drivinit.h | &#x2714; | &#x2714; | All definitions were moved out, it's literally empty. |
| shared    | dxgi.h | &#x2753; | &#x2714; | A fair few definitions, but only for C types so they can be accessed the same way as their C++ equivalent classes |
| shared    | dxgi.idl | &#x2718; | &#x2718; | Extra information and comments for the generated `dxgi.h` |
| shared    | dxgi1_2.h | &#x2718; | | dxgi.h versioned additions. If added, would be merged into one file. (DirectX Graphics Infrastructure version 1.2) |
| shared    | dxgi1_2.idl | &#x2718; | | dxgi.h versioned additions. If added, would be merged into one file. (DirectX Graphics Infrastructure version 1.2) |
| shared    | dxgi1_3.h | &#x2718; | | dxgi.h versioned additions. If added, would be merged into one file. (DirectX Graphics Infrastructure version 1.3) |
| shared    | dxgi1_3.idl | &#x2718; | | dxgi.h versioned additions. If added, would be merged into one file. (DirectX Graphics Infrastructure version 1.3) |
| shared    | dxgi1_4.h | &#x2718; | | dxgi.h versioned additions. If added, would be merged into one file. (DirectX Graphics Infrastructure version 1.4) |
| shared    | dxgi1_4.idl | &#x2718; | | dxgi.h versioned additions. If added, would be merged into one file. (DirectX Graphics Infrastructure version 1.4) |
| shared    | dxgi1_5.h | &#x2718; | | dxgi.h versioned additions. If added, would be merged into one file. (DirectX Graphics Infrastructure version 1.5) |
| shared    | dxgi1_5.idl | &#x2718; | | dxgi.h versioned additions. If added, would be merged into one file. (DirectX Graphics Infrastructure version 1.5) |
| shared    | dxgi1_6.h | &#x2718; | | dxgi.h versioned additions. If added, would be merged into one file. (DirectX Graphics Infrastructure version 1.6) |
| shared    | dxgi1_6.idl | &#x2718; | | dxgi.h versioned additions. If added, would be merged into one file. (DirectX Graphics Infrastructure version 1.6) |
| shared    | dxgicommon.h | &#x2718; | | Nothing to implement |
| shared    | dxgicommon.idl | &#x2718; | | Generator source for header |
| shared    | dxgiformat.h | &#x2718; | | Constants and typedefs |
| shared    | dxgiformat.idl | &#x2718; | | Generator source for header |
| shared    | dxgitype.h | &#x2718; | | Constants and typedefs |
| shared    | dxgitype.idl | &#x2718; | | Generator source for header |
| shared    | ehstorbandmgmt.h | &#x2714; | &#x2718; | Enhanced Storage IOCTL Band Control and Trading Card Games (Trusted Computing Group) Storage Silo |
| shared    | ehstorioctl.h | &#x2714; | &#x2718; | More Enhanced storage IOCTL stuff |
| shared    | emi.h | &#x2714; | &#x2718; | Energy Metering Interfaces |
| shared    | evntprov.h | &#x2718; | | Event API constants and types |
| shared    | evntrace.h | &#x2718; | | Event Tracing constants and types |
| shared    | exposeenums2managed.h | &#x2718; | | Enums for Managed Code (the Visual* family of MS langs). Possibly relevant in the future for Visual Rust. |
| shared    | fltUserStructures.h | &#x2718; | | Only typedefs |
| shared    | fttypes.h | &#x2718; | | Only typedefs |
| shared    | fwpmtypes.h | &#x2718; | | Only typedefs |
| shared    | fwpmtypes.idl | &#x2718; | | Generator sources |
| shared    | fwpstypes.h | &#x2718; | | Only typedefs |
| shared    | fwpstypes.idl | &#x2718; | | Generator sources |
| shared    | fwptypes.h | &#x2718; | | Only typedefs |
| shared    | fwptypes.idl | &#x2718; | | Generator sources |
| shared    | fwpvi.h | &#x2718; | | Literally just constants |
| shared    | GenericUsbFnIoctl.h | &#x2718; | | Just a few control codes |
| shared    | gnssdriver.h | &#x2718; | | Typedefs and constants |
| shared    | gpio.h | &#x2718; | | Control codes |
| shared    | guiddef.h | &#x2714; | &#x2718; | Comparisons and comparator overloads for GUIDs |
| shared    | hbaapi.h | &#x2718; | | Nothing to do here |
| shared    | hidclass.h | &#x2718; | | Nothing to do here |
| shared    | hidpi.h | &#x2718; | | Nothing to do here |
| shared    | hidsdi.h | &#x2718; | | Nothing to do here |
| shared    | hidusage.h | &#x2718; | | Nothing to do here |
| shared    | hvsocket.h | &#x2718; | | Nothing to do here |
| shared    | hwn.h | &#x2718; | | Nothing to do here |
| shared    | ifdef.h | &#x2718; | | Nothing to do here |
| shared    | ifmib.h | &#x2714; | &#x2718; | Only a custom sizeof implementation |
| shared    | iketypes.h | &#x2718; | | Nothing to do here |
| shared    | iketypes.idl | &#x2718; | | Generator sources |
| shared    | in6addr.h | &#x2718; | | Nothing to do here |
| shared    | inaddr.h | &#x2718; | | Nothing to do here |
| shared    | infstr.h | &#x2718; | | TEXT("Nothing to do here") |
| shared    | initguid.h | &#x2718; | | Nothing to do here |
| shared    | intsafe.h | &#x2714; | &#x2718; | Lots of integer funnies! Fallible conversions, with sanity checks and error returns. |
| shared    | ioevent.h | &#x2718; | | Nothing to do here |
| shared    | ip2string.h | &#x2718; | | Nothing to do here |
| shared    | ipifcons.h | &#x2718; | | Nothing to do here |
| shared    | ipmib.h | &#x2714; | &#x2718; | A few custom sizeof implementations |
| shared    | Iprtrmib.h | &#x2714; | &#x2718; | Sizeof and some other helpers and info functions |
| shared    | ipsectypes.h | &#x2718; | | Nothing to do here |
| shared    | ipsectypes.idl | &#x2718; | | Generator Sources |
| shared    | ipv6prefast.h | &#x2718; | | Nothing to do here |
| shared    | iscsierr.h | &#x2718; | | Nothing to do here |
| shared    | IssPer16.h | &#x2718; | | Nothing to do here |
| shared    | kernelspecs.h | &#x2714; | &#x2718; | Macro bodies might suddenly change - they're macros made up of other macros. |
| shared    | ks.h | &#x2714; | &#x2718; | Lots to do |
| shared    | ksamd64.inc | &#x2718; | | Lots of useful info, but nothing to do |
| shared    | ksamd64_stub.inc | &#x2718; | | Includes another include |
| shared    | ksarm.h | &#x2718; | | Only constants |
| shared    | ksarm64.h | &#x2718; | | ksarm with bigger numbers |
| shared    | ksguid.h | &#x2714; | &#x2718; | Fun GUID toys |
| shared    | ksmedia.h | &#x2714; | &#x2718; | Cool media-related stuff |
| shared    | ksproxy.h | &#x2718; | | Only interfaces |
| shared    | ksuuids.h | &#x2718; | | Constants |
| shared    | ktmtypes.h | &#x2714; | &#x2718; | A few bytelength macros|
| shared    | kxamd64.inc | &#x2718; | | Not sure anyone should be touching that |
| shared    | kxamd64_stub.inc | &#x2718; | | imports the previous menace |
| shared    | kxarm.h | &#x2718; | | ARM Assembly masqueraiding as a header file |
| shared    | kxarm64.h | &#x2718; | | ARM64 Assembly masqueraiding as a header file |
| shared    | kxarm64unw.h | &#x2718; | | ARM64 Assembly masqueraiding as a header file |
| shared    | kxarmunw.h | &#x2718; | | ARM Assembly masqueraiding as a header file |
| shared    | lamp.h | &#x2718; | | Nothing to do here |
| shared    | lmcons.h | &#x2718; | | Nothing to do here |
| shared    | lmerr.h | &#x2718; | | Nothing to do here |
| shared    | macamd64.inc | &#x2718; | | More arcane level assembly |
| shared    | Math3DHelper.h | &#x2718; | | Rust has proper math libraries. Not implementing, unless there's no other way to get some of these objects. |
| shared    | minwindef.h | &#x2714; | &#x2714; | Expanded to cover `u16` through `u64` for all functions |
| shared    | mmreg.h | &#x2714; | &#x2718; | Some multimedia & GUID stuff |
| shared    | mprapidef.h | &#x2718; | | Nothing to do here |
| shared    | msapofxproxy.h | &#x2718; | | Nothing to do here |
| shared    | mstcpip.h | &#x2718; | | Nothing to do here, use `std::net::IpAddr` |
| shared    | mswsockdef.h | &#x2714; | &#x2718; | What do you **mean** Microsoft-specific extensions to the Windows Sockets API? Is there a non-MS Windows Socket API? IN MS WINDOWS? |
| shared    | ndisguid.h | &#x2718; | | Nothing to do here |
| shared    | ndkinfo.h | &#x2718; | | Nothing to do here |
| shared    | netevent.h | &#x2718; | | Nothing to do here |
| shared    | netioapi.h | &#x2718; | | These are only API defs, impl lives elsewhere. |
| shared    | netiodef.h | &#x2718; | | Nothing not in `std::net` other than prefixing strings with `\\.\` or `\DEVICE\` |
| shared    | nettypes.h | &#x2718; | | Nothing to do here |
| shared    | nfcdtadev.h | &#x2718; | | Nothing to do here |
| shared    | nfcradiodev.h | &#x2718; | | Nothing to do here |
| shared    | nfcsedev.h | &#x2718; | | This only provides macros to create anonymous structs with sized element and payload data for NFC. Roll your own structs instead. |
| shared    | nfpdev.h | &#x2718; | | Nothing to do here |
| shared    | nldef.h | &#x2718; | | No macros ~~and also not a definition of The Netherlands (NL)~~ |
| shared    | no_sal2.h | &#x2718; | | This undefines everything SAL2 |
| shared    | ntdd1394.h | &#x2718; | | Nothing to do here |
| shared    | ntddbeep.h | &#x2718; | | BEEP |
| shared    | ntddcdrm.h | &#x2718; | | Nothing to do here |
| shared    | ntddcdvd.h | &#x2718; | | Nothing to do here |
| shared    | ntddchgr.h | &#x2718; | | Nothing to do here |
| shared    | ntdddisk.h | &#x2718; | | Some disk geometry stuff, maybe? |
| shared    | ntddkbd.h | &#x2718; | | Would enahnced keyboard detection even add anything? |
| shared    | ntddmmc.h | &#x2718; | | Nothing to do here |
| shared    | ntddmodm.h | &#x2718; | | Nothing to do here |
| shared    | ntddmou.h | &#x2718; | | Nothing to do here |
| shared    | ntddndis.h | &#x2718; | | Nothing to do here |
| shared    | ntddpar.h | &#x2718; | | Nothing to do here |
| shared    | ntddscm.h | &#x2718; | | Nothing to do here |
| shared    | ntddscsi.h | &#x2718; | | Nothing to do here |
| shared    | ntddser.h | &#x2718; | | Nothing to do here |
| shared    | ntddstor.h | &#x2714; | &#x2718; | |
| shared    | ntddtape.h | &#x2718; | | Nothing to do here |
| shared    | ntddtdi.h | &#x2714; | &#x2718; | Not sure if it's worth it even |
| shared    | ntddvdeo.h | &#x2718; | | This one only calculates the size of the data behind a frame buffer pointer... |
| shared    | ntddvol.h | &#x2718; | | Nothing to do here |
| shared    | ntdef.h | &#x2714; | &#x2718; | Cool NT stuff to do I guess |
| shared    | ntdskreg.h | &#x2718; | | Nothing to do here |
| shared    | ntiologc.h | &#x2718; | | Nothing to do here |
| shared    | ntstatus.h | &#x2714; | &#x2718; | HRESULT and WIN32 to NTSTATUS |
| shared    | ntverp.h | &#x2718; | | We have version numbers at home |
| shared    | ntverp.ver | &#x2718; | | Nothing to do here |
| shared    | nvme.h | &#x2718; | | Nothing to do here |
| shared    | packoff.h | &#x2718; | | Nothing to do here |
| shared    | packon.h |&#x2718; | | Nothing to do here |
| shared    | pciprop.h | &#x2714; | &#x2718; | |
| shared    | poclass.h | &#x2718; | | Nothing to do here |
| shared    | poppack.h | &#x2718; | | Nothing to do here |
| shared    | pshpack1.h | &#x2718; | | Nothing to do here |
| shared    | pshpack2.h | &#x2718; | | Nothing to do here |
| shared    | pshpack4.h | &#x2718; | | Nothing to do here |
| shared    | pshpack8.h | &#x2718; | | Nothing to do here |
| shared    | pshpck16.h | &#x2718; | | Nothing to do here |
| shared    | pwm.h | &#x2718; | | Nothing to do here |
| shared    | qos.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | qosobjs.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | qossp.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | reshub.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | rpc.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | rpcasync.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | rpcdce.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | rpcdcep.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | rpcndr.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | rpcnterr.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | rpcsal.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | sal.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | scsi.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | scsiscan.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | sddl.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | sdkddkver.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | sdv_driverspecs.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | secext.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | security.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | sensorsdef.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | sensorsstructures.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | sensorsutils.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | spb.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | specstrings.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | specstrings_strict.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | specstrings_undef.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | srb.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | sspi.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | stralign.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | strsafe.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | suppress.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | tbs.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | tcpestats.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | tcpmib.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | tdi.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | tdiinfo.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | TraceLoggingActivity.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | TraceLoggingProvider.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | traffic.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | transportsettingcommon.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | tvout.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | udpmib.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | unexposeenums2managed.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | usb.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | usb100.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | usb200.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | usbdi.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | usbfnbase.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | usbioctl.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | usbiodef.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | usbprint.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | usbscan.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | usbspec.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | uuids.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | vhf.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | vmgenerationcounter.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | warning.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | wdmguid.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | winapifamily.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | winbio_err.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | winbio_ioctl.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | winbio_types.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | windef.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | windot11.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | windowsx.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | winerror.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | winpackagefamily.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | winsmcrd.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | winusbio.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | wlantypes.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | wmistr.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | wnnc.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ws2def.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ws2ipdef.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | wtypes.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | WTypes.Idl | &#x2714; | &#x2718; | Not yet checked |
| shared    | WTypesbase.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | WTypesbase.Idl | &#x2714; | &#x2718; | Not yet checked |
