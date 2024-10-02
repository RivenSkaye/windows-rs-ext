# Headers from the Windows SDK

This page should list all of the headers in the default `Include/<version>/shared` folder for the Windows SDK. The UCRT headers were taken as a starting point, the other directories in the versioned SDK might at some point be added, if they prove to contain more useful things not provided through the existing projections.

Entries for which the headers have not yet been tracked are subject to become unplanned if it turns out they don't fit the project. Driver and device interfaces will not be supported here unless someone else comes on board to maintain them. There's some stuff not (yet) worth using.

| Directory | File | Planned  | Implemented | Notes |
|-----------|------|----------|-------------|-------|
| shared/ndis | *  | &#x2718; | | Network Driver stuff, out of scope |
| shared/netcx | * | &#x2718; | | Network Adapter Class Extensions |
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
| shared    | ehstorbandmgmt.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ehstorioctl.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | emi.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | evntprov.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | evntrace.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | exposeenums2managed.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | fltUserStructures.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | fttypes.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | fwpmtypes.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | fwpmtypes.idl | &#x2714; | &#x2718; | Not yet checked |
| shared    | fwpstypes.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | fwpstypes.idl | &#x2714; | &#x2718; | Not yet checked |
| shared    | fwptypes.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | fwptypes.idl | &#x2714; | &#x2718; | Not yet checked |
| shared    | fwpvi.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | GenericUsbFnIoctl.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | gnssdriver.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | gpio.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | guiddef.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | hbaapi.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | hidclass.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | hidpi.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | hidsdi.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | hidusage.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | hvsocket.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | hwn.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ifdef.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ifmib.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | iketypes.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | iketypes.idl | &#x2714; | &#x2718; | Not yet checked |
| shared    | in6addr.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | inaddr.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | infstr.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | initguid.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | intsafe.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ioevent.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ip2string.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ipifcons.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ipmib.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | Iprtrmib.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ipsectypes.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ipsectypes.idl | &#x2714; | &#x2718; | Not yet checked |
| shared    | ipv6prefast.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | iscsierr.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | IssPer16.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | kernelspecs.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ks.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ksamd64.inc | &#x2714; | &#x2718; | Not yet checked |
| shared    | ksamd64_stub.inc | &#x2714; | &#x2718; | Not yet checked |
| shared    | ksarm.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ksarm64.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ksguid.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ksmedia.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ksproxy.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ksuuids.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ktmtypes.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | kxamd64.inc | &#x2714; | &#x2718; | Not yet checked |
| shared    | kxamd64_stub.inc | &#x2714; | &#x2718; | Not yet checked |
| shared    | kxarm.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | kxarm64.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | kxarm64unw.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | kxarmunw.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | lamp.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | lmcons.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | lmerr.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | macamd64.inc | &#x2714; | &#x2718; | Not yet checked |
| shared    | Math3DHelper.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | minwindef.h | &#x2714; | &#x2714; | Expanded to cover `u16` through `u64` for all functions |
| shared    | mmreg.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | mprapidef.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | msapofxproxy.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | mstcpip.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | mswsockdef.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ndis | &#x2714; | &#x2718; | Not yet checked |
| shared    | ndisguid.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ndkinfo.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | netcx | &#x2714; | &#x2718; | Not yet checked |
| shared    | netevent.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | netioapi.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | netiodef.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | nettypes.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | nfcdtadev.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | nfcradiodev.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | nfcsedev.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | nfpdev.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | nldef.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | no_sal2.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntdd1394.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntddbeep.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntddcdrm.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntddcdvd.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntddchgr.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntdddisk.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntddkbd.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntddmmc.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntddmodm.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntddmou.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntddndis.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntddpar.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntddscm.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntddscsi.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntddser.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntddstor.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntddtape.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntddtdi.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntddvdeo.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntddvol.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntdef.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntdskreg.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntiologc.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntstatus.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntverp.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | ntverp.ver | &#x2714; | &#x2718; | Not yet checked |
| shared    | nvme.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | packoff.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | packon.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | pciprop.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | poclass.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | poppack.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | pshpack1.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | pshpack2.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | pshpack4.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | pshpack8.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | pshpck16.h | &#x2714; | &#x2718; | Not yet checked |
| shared    | pwm.h | &#x2714; | &#x2718; | Not yet checked |
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