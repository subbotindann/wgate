#include <stdint.h>
#include <stdio.h>

int main(void) {
    FindWindowW(L"osu!", 0);
    GetForegroundWindow();
    SetForegroundWindow(hwnd);
    GetWindowRect(hwnd, rectOut);
    GetClientRect(hwnd, clientRectOut);
    ClientToScreen(hwnd, pointOut);
    ScreenToClient(hwnd, pointOut);
    GetDC(hwnd);
    ReleaseDC(hwnd, hdc);
    GetSystemMetrics(0);

    SendInput(2, inputs, 40);
    mouse_event(1, 0, 0, 0, 0);
    keybd_event(90, 0, 0, 0);
    keybd_event(90, 0, 2, 0);

    return 0;
}
