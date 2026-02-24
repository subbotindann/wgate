#include <stdint.h>
#include <stdio.h>

int main(void) {
    RegisterClassExW(wc);
    CreateWindowExW(0, L"cls", L"title", 0, 10, 10, 640, 480, 0, 0, 0, 0);
    DefWindowProcW(hwnd, WM_CREATE, 0, 0);
    ShowWindow(hwnd, 1);
    UpdateWindow(hwnd);
    GetClientRect(hwnd, rectOut);
    AdjustWindowRectEx(rectOut, 0, false, 0);
    SetWindowTextW(hwnd, L"updated");
    PeekMessageW(msg, 0, 0, 0, 1);
    GetMessageW(msg, 0, 0, 0);
    TranslateMessage(msg);
    DispatchMessageW(msg);
    PostMessageW(hwnd, WM_CLOSE, 0, 0);
    SendMessageW(hwnd, WM_SIZE, 0, 0);
    DestroyWindow(hwnd);

    WM_CREATE();
    WM_DESTROY();
    WM_PAINT();
    WM_SIZE();
    WM_CLOSE();
    WM_QUIT();

    return 0;
}
