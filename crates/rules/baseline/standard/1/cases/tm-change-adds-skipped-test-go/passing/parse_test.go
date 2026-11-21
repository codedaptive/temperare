package parse

import (
	"strconv"
	"testing"
)

func parsePort(text string) (uint16, error) {
	port, err := strconv.ParseUint(text, 10, 16)
	return uint16(port), err
}

func TestReadsAPort(t *testing.T) {
	if port, err := parsePort("8080"); err != nil || port != 8080 {
		t.Fatalf("got %d, %v", port, err)
	}
}

func TestRefusesAPortOutOfRange(t *testing.T) {
	if _, err := parsePort("70000"); err == nil {
		t.Fatal("70000 was read as a port")
	}
}
