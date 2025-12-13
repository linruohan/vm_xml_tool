## Xen with fullvirt loader

```xml

<os>
    <type>hvm</type>
    <loader>/usr/lib/xen/boot/hvmloader</loader>
    <boot dev='hd'/>
</os>
```

## QEMU with default firmware, serial console and SMBIOS

```xml

<os>
    <type>hvm</type>
    <boot dev='cdrom'/>
    <bootmenu enable='yes' timeout='3000'/>
    <smbios mode='sysinfo'/>
    <bios useserial='yes' rebootTimeout='0'/>
</os>
```

## QEMU with UEFI manual firmware and secure boot

```xml

<os>
    <type>hvm</type>
    <loader readonly='yes' secure='yes' type='pflash'>/usr/share/OVMF/OVMF_CODE.fd</loader>
    <nvram template='/usr/share/OVMF/OVMF_VARS.fd'>/var/lib/libvirt/nvram/guest_VARS.fd</nvram>
    <boot dev='hd'/>
</os>
```

## QEMU with UEFI manual firmware, secure boot and with NVRAM type 'file'

```xml

<os>
    <type>hvm</type>
    <loader readonly='yes' secure='yes' type='pflash'>/usr/share/OVMF/OVMF_CODE.fd</loader>
    <nvram type='file' template='/usr/share/OVMF/OVMF_VARS.fd'>
        <source file='/var/lib/libvirt/nvram/guest_VARS.fd'/>
    </nvram>
    <boot dev='hd'/>
</os>
```

## QEMU with UEFI manual firmware, secure boot and with network backed NVRAM'

```xml

<os>
    <type>hvm</type>
    <loader readonly='yes' secure='yes' type='pflash'>/usr/share/OVMF/OVMF_CODE.fd</loader>
    <nvram type='network'>
        <source protocol='iscsi' name='iqn.2013-07.com.example:iscsi-nopool/0'>
            <host name='example.com' port='6000'/>
            <auth username='myname'>
                <secret type='iscsi' usage='mycluster_myname'/>
            </auth>
        </source>
    </nvram>
    <boot dev='hd'/>
</os>
```

## QEMU with automatic UEFI firmware and secure boot

```xml

<os firmware='efi'>
    <type>hvm</type>
    <loader secure='yes'/>
    <boot dev='hd'/>
</os>
```

## QEMU with automatic UEFI stateless firmware for AMD SEV

```xml

<os firmware='efi'>
    <type>hvm</type>
    <loader stateless='yes'/>
    <boot dev='hd'/>
</os>
```