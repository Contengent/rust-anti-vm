# rust-anti-vm
Some ok methods of detecting vms, terribly written and POC in rust. <br/>
✅ = Done <br/>
❌ = WIP

## My heuristic based anti-vm techniques:
### NETWORK BASED
- ping gateway address to see if response is unusually fast [1] ❌
- 'wget [gateway]' to see if it is blocked [2] ✅

### HUMAN-LIKE BEHAVIOUR
- check browser cache size [3] ❌
- check for bios battery [4] ❌

### CPU
- "Check vendor ID string via CPUID instruction" [5]  ✅

## notes:
[1] Can be caused by professional, pro-sumer grade, or even some newer regular level networking equipment. Can be used with other techniques. <br/><br/>
[2] Can be caused by guest network, or strict network access. Should be used in tandem with other techniques. <br/><br/>
[3] Can be made more effective by checking the actual cache data. <br/><br/>
[4] A little... odd? You can disable network based date/time syncing, and check if the time was kept track of whilst the computer was off.
	There is likely a better way to check for the presence of a CMOS battery since this method requires access to the 'hosts' file. It
	also requires that the computer be rebooted, requiring the malware to preserve a file on the victim PC. Ultimately defeating the
	purpose of being an ANTI-VM technique. However, there still might be a method, or even a use for this method, that could be used.
	TBH, this method has A LOT of caveats. It's Honestly just a POC at this point lol. <br/><br/>
[5] This method is already well known. However I wanted to attempt to re-write it in rust. I don't have any reason other than it seems like fun. <br/>


### more notes:
All of these methods were developed and checked on a QEMU/KVM VM, using virt-manager and gpu-passthrough via OVMF (Felt like I had to be specific).
I have yet to test if these are actual functioning anti-vm methods, or if they just work for me and my systems because they are setup poorly, or edge
cases. I do plan on getting some friends to run them on baremetal windows machines to see their accuracy. I also do plan on personally going in and
testing them on other virtual machine platforms.
