CPU model and topology

Requirements for CPU model, its features and topology can be specified using the following collection of elements. Since
0.7.5

```xml

<cpu match='exact'>
    <model fallback='allow'>core2duo</model>
    <vendor>Intel</vendor>
    <topology sockets='1' dies='1' clusters='1' cores='2' threads='1'/>
    <cache level='3' mode='emulate'/>
    <maxphysaddr mode='emulate' bits='42'/>
    <feature policy='disable' name='lahf_lm'/>
</cpu>
```

```xml

<cpu mode='host-model'>
    <model fallback='forbid'/>
    <topology sockets='1' dies='1' clusters='1' cores='2' threads='1'/>
</cpu>
```

```xml

<cpu mode='host-passthrough' migratable='off'>
    <cache mode='passthrough'/>
    <maxphysaddr mode='passthrough' limit='39'/>
    <feature policy='disable' name='lahf_lm'/>
```

```xml

<cpu mode='maximum' migratable='off'>
    <cache mode='passthrough'/>
    <feature policy='disable' name='lahf_lm'/>
```

In case no restrictions need to be put on CPU model and its features, a simpler cpu element can be used. Since 0.7.6

```xml

<cpu>
    <topology sockets='1' dies='1' clusters='1' cores='2' threads='1'/>
</cpu>
```

cpu

    The cpu element is the main container for describing guest CPU requirements. Its match attribute specifies how strictly the virtual CPU provided to the guest matches these requirements. Since 0.7.6 the match attribute can be omitted if topology is the only element within cpu. Possible values for the match attribute are:

    minimum

        The specified CPU model and features describes the minimum requested CPU. A better CPU will be provided to the guest if it is possible with the requested hypervisor on the current host. This is a constrained host-model mode; the domain will not be created if the provided virtual CPU does not meet the requirements.
    exact

        The virtual CPU provided to the guest should exactly match the specification. If such CPU is not supported, libvirt will refuse to start the domain.
    strict

        The domain will not be created unless the host CPU exactly matches the specification. This is not very useful in practice and should only be used if there is a real reason.

    Since 0.8.5 the match attribute can be omitted and will default to exact. Sometimes the hypervisor is not able to create a virtual CPU exactly matching the specification passed by libvirt. Since 3.2.0, an optional check attribute can be used to request a specific way of checking whether the virtual CPU matches the specification. It is usually safe to omit this attribute when starting a domain and stick with the default value. Once the domain starts, libvirt will automatically change the check attribute to the best supported value to ensure the virtual CPU does not change when the domain is migrated to another host. The following values can be used:

    none

        Libvirt does no checking and it is up to the hypervisor to refuse to start the domain if it cannot provide the requested CPU. With QEMU this means no checking is done at all since the default behavior of QEMU is to emit warnings, but start the domain anyway.
    partial

        Libvirt will check the guest CPU specification before starting a domain, but the rest is left on the hypervisor. It can still provide a different virtual CPU.
    full

        The virtual CPU created by the hypervisor will be checked against the CPU specification and the domain will not be started unless the two CPUs match.

    Since 0.9.10, an optional mode attribute may be used to make it easier to configure a guest CPU to be as close to host CPU as possible. Possible values for the mode attribute are:

    custom

        In this mode, the cpu element describes the CPU that should be presented to the guest. This is the default when no mode attribute is specified. This mode makes it so that a persistent guest will see the same hardware no matter what host the guest is booted on.
    host-model

        The host-model mode is essentially a shortcut to copying host-model CPU definition from domain capabilities XML into domain XML. Since the CPU definition is copied just before starting a domain, exactly the same XML can be used on different hosts while still providing the best guest CPU each host supports. The match attribute can't be used in this mode. Specifying CPU model is not supported either, but model's fallback attribute may still be used. Using the feature element, specific flags may be enabled or disabled specifically in addition to the host model. This may be used to fine tune features that can be emulated. (Since 1.1.1)

        Libvirt does not model every aspect of each CPU so the guest CPU will not match the host CPU exactly. On the other hand, the ABI provided to the guest is reproducible. During migration, complete CPU model definition is transferred to the destination host so the migrated guest will see exactly the same CPU model for the running instance of the guest, even if the destination host contains more capable CPUs or newer kernel; but shutting down and restarting the guest may present different hardware to the guest according to the capabilities of the new host.

        Prior to libvirt 3.2.0 and QEMU 2.9.0 detection of the host CPU model via QEMU is not supported. Thus the CPU configuration created using host-model may not work as expected. Since 3.2.0 and QEMU 2.9.0 this mode works the way it was designed and it is indicated by the fallback attribute set to forbid in the host-model CPU definition advertised in domain capabilities XML. When fallback attribute is set to allow in the domain capabilities XML, it is recommended to use custom mode with just the CPU model from the host capabilities XML.

        Since 1.2.11 PowerISA allows processors to run VMs in binary compatibility mode supporting an older version of ISA. Libvirt on PowerPC architecture uses the host-model to signify a guest mode CPU running in binary compatibility mode. Example: When a user needs a power7 VM to run in compatibility mode on a Power8 host, this can be described in XML as follows:

        <cpu mode='host-model'>
          <model>power7</model>
        </cpu>
        ...

    host-passthrough

        With this mode, the CPU visible to the guest should be exactly the same as the host CPU even in the aspects that libvirt does not understand. Though the downside of this mode is that the guest environment cannot be reproduced on different hardware. Thus, if you hit any bugs, you are on your own. Further details of that CPU can be changed using feature elements. Migration of a guest using host-passthrough is dangerous if the source and destination hosts are not identical in both hardware, QEMU version, microcode version and configuration. If such a migration is attempted then the guest may hang or crash upon resuming execution on the destination host. Depending on hypervisor version the virtual CPU may or may not contain features which may block migration even to an identical host. Since 6.5.0 optional migratable attribute may be used to explicitly request such features to be removed from (on) or kept in (off) the virtual CPU. This attribute does not make migration to another host safer: even with migratable='on' migration will be dangerous unless both hosts are identical as described above.
    maximum

        When running a guest with hardware virtualization this CPU model is functionally identical to host-passthrough, so refer to the docs above.

        When running a guest with CPU emulation, this CPU model will enable the maximum set of features that the emulation engine is able to support. Note that even with migratable='on' migration will be dangerous unless both hosts are running identical versions of the emulation code.

        Since 7.1.0 with the QEMU driver.

    Both host-model and host-passthrough modes make sense when a domain can run directly on the host CPUs (for example, domains with type kvm or hvf). The actual host CPU is irrelevant for domains with emulated virtual CPUs (such as domains with type qemu). However, for backward compatibility host-model may be implemented even for domains running on emulated CPUs in which case the best CPU the hypervisor is able to emulate may be used rather then trying to mimic the host CPU model.

    If an application does not care about a specific CPU, just wants the best feature set without a need for migration compatibility, the maximum model is a good choice on hypervisors where it is available.

model

    The content of the model element specifies CPU model requested by the guest. The list of available CPU models and their definition can be found in directory cpu_map, installed in libvirt's data directory. If a hypervisor is not able to use the exact CPU model, libvirt automatically falls back to a closest model supported by the hypervisor while maintaining the list of CPU features. Since 0.9.10, an optional fallback attribute can be used to forbid this behavior, in which case an attempt to start a domain requesting an unsupported CPU model will fail. Supported values for fallback attribute are: allow (this is the default), and forbid. The optional vendor_id attribute ( Since 0.10.0 ) can be used to set the vendor id seen by the guest. It must be exactly 12 characters long. If not set the vendor id of the host is used. Typical possible values are "AuthenticAMD" and "GenuineIntel".

vendor

    Since 0.8.3 the content of the vendor element specifies CPU vendor requested by the guest. If this element is missing, the guest can be run on a CPU matching given features regardless on its vendor. The list of supported vendors can be found in cpu_map/*_vendors.xml.

topology

    The topology element specifies requested topology of virtual CPU provided to the guest. Its attributes sockets, dies (Since 6.1.0), clusters (Since 10.1.0), cores, and threads accept non-zero positive integer values. They refer to the total number of CPU sockets, number of dies per socket, number of clusters per die, number of cores per cluster, and number of threads per core, respectively. The dies and clusters attributes are optional and will default to 1 if omitted, while the other attributes are all mandatory. Hypervisors may require that the maximum number of vCPUs specified by the cpus element equals to the number of vcpus resulting from the topology. Moreover, not all architectures and machine types support specifying a value other than 1 for all attributes.

feature

    The cpu element can contain zero or more feature elements used to fine-tune features provided by the selected CPU model. The list of known feature names can be found in the same file as CPU models. The meaning of each feature element depends on its policy attribute, which has to be set to one of the following values:

    force

        The virtual CPU will claim the feature is supported regardless of it being supported by host CPU.
    require

        Guest creation will fail unless the feature is supported by the host CPU or the hypervisor is able to emulate it.
    optional

        The feature will be supported by virtual CPU if and only if it is supported by host CPU.
    disable

        The feature will not be supported by virtual CPU.
    forbid

        Guest creation will fail if the feature is supported by host CPU.

    Since 0.8.5 the policy attribute can be omitted and will default to require.

    Individual CPU feature names are specified as part of the name attribute. For example, to explicitly specify the 'pcid' feature with Intel IvyBridge CPU model:

    ...
    <cpu match='exact'>
      <model fallback='forbid'>IvyBridge</model>
      <vendor>Intel</vendor>
      <feature policy='require' name='pcid'/>
    </cpu>
    ...

deprecated_features

    Since 11.0.0, S390 guests may utilize the deprecated_features attribute to specify toggling of CPU model features that are flagged as deprecated by the hypervisor. When this attribute is set to off, the active guest XML will reflect the respective features with the disable policy. When this attribute is set to on, the respective features will be enabled.

cache

    Since 3.3.0 the cache element describes the virtual CPU cache. If the element is missing, the hypervisor will use a sensible default.

    level

        This optional attribute specifies which cache level is described by the element. Missing attribute means the element describes all CPU cache levels at once. Mixing cache elements with the level attribute set and those without the attribute is forbidden.
    mode

        The following values are supported:

        emulate

            The hypervisor will provide a fake CPU cache data.
        passthrough

            The real CPU cache data reported by the host CPU will be passed through to the virtual CPU.
        disable

            The virtual CPU will report no CPU cache of the specified level (or no cache at all if the level attribute is missing).

maxphysaddr

    Since 8.7.0 the maxphysaddr element describes the virtual CPU address size in bits. The hypervisor default is used if the element is missing.

    mode

        This mandatory attribute specifies how the address size is presented. The follow modes are supported:

        passthrough

            The number of physical address bits reported by the host CPU will be passed through to the virtual CPUs
        emulate

            The hypervisor will define a specific value for the number of bits of physical addresses via the bits attribute, (optional since 9.2.0) The number of bits cannot exceed the number of physical address bits supported by the hypervisor.

    bits

        The bits attribute is mandatory if the mode attribute is set to emulate and specifies the virtual CPU address size in bits.
    limit

        The limit attribute can be used to restrict the maximum value of address bits for passthrough mode, i.e. in case the host CPU reports more bits than that, limit is used. Since 9.3.0

