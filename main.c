#include <stdio.h>
#include "getAbsPos.h"
#include <pthread.h>
#include <time.h>
#include <unistd.h>
#include "absMove.h"
#include "keyboard.h"

typedef struct
{
    int left;
    int top;
    int right;
    int bottom;
} RECT;

void MAIN_INIT(){
    init_tablet();
    init_layer_shell();
    init_virtual_mouse();
    pthread_t cursor_pos_thread;
    pthread_create(&cursor_pos_thread, NULL, update_cursor_pos, NULL); 
    initilize_keyboard();
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
            move_absolute(right, cursor_y);
            printf("right \n");
        }
        if (cursor_x < left){
            move_absolute(left, cursor_y);
            printf("left \n");
        }
        if (cursor_y < top){
            move_absolute(cursor_x, top);
            printf("top \n");
        }
        if (cursor_y > bottom){
            move_absolute(cursor_x, bottom);
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

int main(){
    printf("Hello world! \n");
    MAIN_INIT();
    sleep(2);
    mouseClick(BTN_LEFT);
    
    MAIN_DESTROY();
    return 0;
}