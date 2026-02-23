#include <stdio.h>
#include "getAbsPos.h"
#include <pthread.h>
#include <time.h>
#include <unistd.h>
#include "absMove.h"
#include "keyboard.h"
#include "structures.h"




void MAIN_INIT(){
    init_tablet();
    init_layer_shell();
    init_virtual_mouse();
    initilize_keyboard();
    pthread_t cursor_pos_thread;
    pthread_create(&cursor_pos_thread, NULL, update_cursor_pos, NULL); 
    
}
void MAIN_DESTROY(){
    destroy_tablet();
    destroy_layer_shell();
    destroy_virtual_mouse();
    destroy_keyboard();
}

void* restrict_cursor(void* arg){
    RECT* rect  = (RECT*) arg;
    int right = rect -> right;
    int left = rect -> left;
    int bottom = rect -> bottom;
    int top = (*rect).top;
    printf("top: %d \n", rect->top);
    while (true){
        if (cursor_x > right){
            SetCursorPos(right, cursor_y);
            printf("right \n");
        }
        if (cursor_x < left){
            SetCursorPos(left, cursor_y);
            printf("left \n");
        }
        if (cursor_y < top){
            SetCursorPos(cursor_x, top);
            printf("top \n");
        }
        if (cursor_y > bottom){
            SetCursorPos(cursor_x, bottom);
            printf("bottom \n");
        }
    }
}

bool ClipCursor(RECT rect){
    printf("top1: %d \n", rect.top);
    RECT copy = rect;
    pthread_t clip_cursor_thread;
    pthread_create(&clip_cursor_thread, NULL, restrict_cursor, &copy);
    sleep(1);
    return 0;
}

bool GetCursorPos(POINT *point){
    (*point).x = cursor_x;
    (*point).y = cursor_y;
    return 1;
}

UINT SendInput(UINT cInputs, INPUT inputs[], int cbSize){
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
                emit(EV_KEY, input.ki.wVk, is_pressed);
                sleep(1);
                break;
            default:
                return 1;

        }
        sleep(1);
    }
    return 0;
}


int main(){
    printf("Hello world! \n");
    MAIN_INIT();
    
    
    INPUT inputs[2] = {0};
    
    inputs[0].type = INPUT_KEYBOARD;
    inputs[0].ki.wVk = KEY_LEFTMETA;  // Левая клавиша Windows
    inputs[0].ki.dwFlags = 0;     // Нажатие
    printf("%d", );
    //SendInput(2, inputs, sizeof(INPUT));

    
    MAIN_DESTROY();
    return 0;
}