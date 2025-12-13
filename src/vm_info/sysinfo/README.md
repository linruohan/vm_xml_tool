
SMBIOS System Information

Some hypervisors allow control over what system information is presented to the guest (for example, SMBIOS fields can be populated by a hypervisor and inspected via the dmidecode command in the guest). The optional sysinfo element covers all such categories of information. Since 0.8.7

```xml

<os>
    <smbios mode='sysinfo'/>
    ...
</os>
<sysinfo type='smbios'>
<bios>
    <entry name='vendor'>LENOVO</entry>
</bios>
<system>
    <entry name='manufacturer'>Fedora</entry>
    <entry name='product'>Virt-Manager</entry>
    <entry name='version'>0.9.4</entry>
</system>
<baseBoard>
    <entry name='manufacturer'>LENOVO</entry>
    <entry name='product'>20BE0061MC</entry>
    <entry name='version'>0B98401 Pro</entry>
    <entry name='serial'>W1KS427111E</entry>
</baseBoard>
<chassis>
    <entry name='manufacturer'>Dell Inc.</entry>
    <entry name='version'>2.12</entry>
    <entry name='serial'>65X0XF2</entry>
    <entry name='asset'>40000101</entry>
    <entry name='sku'>Type3Sku1</entry>
</chassis>
<oemStrings>
    <entry>myappname:some arbitrary data</entry>
    <entry>otherappname:more arbitrary data</entry>
</oemStrings>
</sysinfo>
<sysinfo type='fwcfg'>
<entry name='opt/com.example/name'>example value</entry>
<entry name='opt/com.coreos/config' file='/tmp/provision.ign'/>
</sysinfo>
```

The sysinfo element has a mandatory attribute type that determine the layout of sub-elements, with supported values of:

smbios

    Sub-elements call out specific SMBIOS values, which will affect the guest if used in conjunction with the smbios sub-element of the os element (see Operating system booting). Each sub-element of sysinfo names a SMBIOS block, and within those elements can be a list of entry elements that describe a field within the block. The following blocks and entries are recognized:

    bios

        This is block 0 of SMBIOS, with entry names drawn from:

        vendor

            BIOS Vendor's Name
        version

            BIOS Version
        date

            BIOS release date. If supplied, is in either mm/dd/yy or mm/dd/yyyy format. If the year portion of the string is two digits, the year is assumed to be 19yy.
        release

            System BIOS Major and Minor release number values concatenated together as one string separated by a period, for example, 10.22.

    system

        This is block 1 of SMBIOS, with entry names drawn from:

        manufacturer

            Manufacturer of BIOS
        product

            Product Name
        version

            Version of the product
        serial

            Serial number
        uuid

            Universal Unique ID number. If this entry is provided alongside a top-level uuid element (see General metadata), then the two values must match.
        sku

            SKU number to identify a particular configuration.
        family

            Identify the family a particular computer belongs to.

    baseBoard

        This is block 2 of SMBIOS. This element can be repeated multiple times to describe all the base boards; however, not all hypervisors necessarily support the repetition. The element can have the following children:

        manufacturer

            Manufacturer of BIOS
        product

            Product Name
        version

            Version of the product
        serial

            Serial number
        asset

            Asset tag
        location

            Location in chassis

        NB: Incorrectly supplied entries for the bios, system or baseBoard blocks will be ignored without error. Other than uuid validation and date format checking, all values are passed as strings to the hypervisor driver.
    chassis

        Since 4.1.0, this is block 3 of SMBIOS, with entry names drawn from:

        manufacturer

            Manufacturer of Chassis
        version

            Version of the Chassis
        serial

            Serial number
        asset

            Asset tag
        sku

            SKU number

    oemStrings

        This is block 11 of SMBIOS. This element should appear once and can have multiple entry child elements, each providing arbitrary string data. There are no restrictions on what data can be provided in the entries, however, if the data is intended to be consumed by an application in the guest, it is recommended to use the application name as a prefix in the string. ( Since 4.1.0 )

fwcfg

    Some hypervisors provide unified way to tweak how firmware configures itself, or may contain tables to be installed for the guest OS, for instance boot order, ACPI, SMBIOS, etc.

    It even allows users to define their own config blobs. In case of QEMU, these then appear under domain's sysfs (if the guest kernel has FW_CFG_SYSFS config option enabled), under /sys/firmware/qemu_fw_cfg. Note, that these values apply regardless the <smbios/> mode under <os/>. Since 6.5.0

    Please note that because of limited number of data slots use of fwcfg is strongly discouraged and <oemStrings/> should be used instead.

    <sysinfo type='fwcfg'>
      <entry name='opt/com.example/name'>example value</entry>
      <entry name='opt/com.example/config' file='/tmp/provision.ign'/>
    </sysinfo>

    The sysinfo element can have multiple entry child elements. Each element then has mandatory name attribute, which defines the name of the blob and must begin with opt/ and to avoid clashing with other names is advised to be in form opt/$RFQDN/$name where $RFQDN is a reverse fully qualified domain name you control. Then, the element can either contain the value (to set the blob value directly), or file attribute (to set the blob value from the file).

