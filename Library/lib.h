#include <stdio.h>
#include "getAbsPos.h"
#include <pthread.h>
#include <time.h>
#include <unistd.h>
#include "absMove.h"
#include "keyboard.h"
//#include "structures.h"
#include "getKeyState.h"

extern void MAIN_INIT(){
    init_tablet();
    init_layer_shell();
    init_virtual_mouse();
    initilize_keyboard();
    init_libinput();
    pthread_t check_buttons_thread;
    pthread_t cursor_pos_thread;
    pthread_create(&cursor_pos_thread, NULL, update_cursor_pos, NULL); 
    pthread_create(&check_buttons_thread, NULL, check_buttons, NULL);
    
}
extern void MAIN_DESTROY(){
    destroy_tablet();
    destroy_layer_shell();
    destroy_virtual_mouse();
    destroy_keyboard();
    destroy_libinput();
}

extern bool GetCursorPos(POINT *point){
    (*point).x = cursor_x;
    (*point).y = cursor_y;
    return 1;
}

extern UINT SendInput(UINT cInputs, INPUT inputs[], int cbSize){
    for (int i = 0; i < cInputs; i++){
        INPUT input = inputs[i];
        switch (input.type){
            case (0):
                switch (input.mi.dwFlags){
                    case (MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE):
                        SetCursorPos(input.mi.dx, input.mi.dy);
                        break;
                    case (MOUSEEVENTF_MOVE):
                        mouseMove(input.mi.dx, input.mi.dy);
                        break;
                    case (MOUSEEVENTF_LEFTDOWN):
                        emit_mouse(EV_KEY, BTN_LEFT, 1);
                        sync_mouse();
                        break;
                    case (MOUSEEVENTF_LEFTUP):
                        emit_mouse(EV_KEY, BTN_LEFT, 0);
                        sync_mouse();
                        break;
                    case (MOUSEEVENTF_RIGHTDOWN):
                        emit_mouse(EV_KEY, BTN_RIGHT, 1);
                        sync_mouse();
                        break;
                    case (MOUSEEVENTF_RIGHTUP):
                        emit_mouse(EV_KEY, BTN_RIGHT, 0);
                        sync_mouse();
                        break;
                    case (MOUSEEVENTF_WHEEL):
                        emit_mouse(EV_REL, REL_WHEEL, input.mi.mouseData);
                        sync_mouse();
                        break;
                    case (MOUSEEVENTF_HWHEEL):
                        emit_mouse(EV_REL, REL_HWHEEL, input.mi.mouseData);
                        sync_mouse();
                        break;
                    
                }
                
                break;
                
            case (1):
                bool is_pressed = 0;
                if (input.ki.dwFlags == NULL){
                    is_pressed = 1;
                }
                printf("Pressing");
                emit(EV_KEY, winapi_to_linux_key(input.ki.wVk), is_pressed);
                break;
            default:
                return 1;

        }
    }
    return 0;
}