#include <pci/pci.h>
#include <stdio.h>

void show_dev_class(u16 dev_class) {
    switch (dev_class) {
    case PCI_CLASS_BRIDGE_HOST:
        printf("Host bridge");
        break;
    case PCI_CLASS_DISPLAY_VGA:
        printf("VGA compatible controller");
        break;
    case PCI_CLASS_BRIDGE_PCI:
        printf("PCI bridge");
        break;
    case PCI_CLASS_MULTIMEDIA_AUDIO_DEV:
        printf("Audio device");
        break;
    case PCI_CLASS_SERIAL_USB:
        printf("USB controller");
        break;
    case PCI_CLASS_BRIDGE_ISA:
        printf("ISA bridge");
        break;
    case PCI_CLASS_STORAGE_SATA:
        printf("SATA controller");
        break;
    case PCI_CLASS_SERIAL_SMBUS:
        printf("SMBus controller");
        break;
    case PCI_CLASS_COMMUNICATION_OTHER:
        printf("Communication controller");
        break;
    case PCI_CLASS_NETWORK_ETHERNET:
        printf("Ethernet controller");
        break;
    case PCI_CLASS_NETWORK_OTHER:
        printf("Network controller");
        break;
    default:
        printf("Unknown device class");
        break;
    }
}

extern void print_pci_name(int vendor, int device);

void print_device(struct pci_dev *p) {
    printf("%02x:%02x.%x ", p->bus, p->dev, p->func);
    show_dev_class(p->device_class);
    printf(": ");
    fflush(stdout);
    print_pci_name(p->vendor_id, p->device_id);
    printf(" [%04x:%04x]\n", p->vendor_id, p->device_id);
    fflush(stdout);
}

void scan_devices(struct pci_access *pacc) {
    pci_scan_bus(pacc);
    for (struct pci_dev *p = pacc->devices; p; p = p->next) {
        print_device(p);
    }
}

void lspci() {
    struct pci_access *pacc;
    pacc = pci_alloc();
    pci_init(pacc);
    scan_devices(pacc);
    pci_cleanup(pacc);
}

int main() {
    lspci();
    return 0;
}
